use crate::request::totp::{DetailResponse, EditIssuerRequestParams, EditUsernameRequestParams};
use application_database::account::access_token;
use application_database::tool::totp;
use application_kernel::result::{Error, Result};
use std::collections::BTreeSet;
use totp_rs::{Secret, TOTP};
use tracing::error;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct SortItem {
    pub id: u64,
    pub sort: u32,
}

impl From<&SortItem> for totp::SortItem {
    fn from(item: &SortItem) -> Self {
        Self {
            id: item.id,
            sort: item.sort,
        }
    }
}

trait TotpSortStore {
    async fn owned_ids(&self, user_id: u64) -> Result<Vec<u64>>;

    async fn replace_all_sorts(&self, user_id: u64, items: &[SortItem]) -> Result<()>;
}

struct DatabaseTotpSortStore;

impl TotpSortStore for DatabaseTotpSortStore {
    async fn owned_ids(&self, user_id: u64) -> Result<Vec<u64>> {
        Ok(totp::all(user_id)
            .await?
            .into_iter()
            .map(|item| item.id)
            .collect())
    }

    async fn replace_all_sorts(&self, user_id: u64, items: &[SortItem]) -> Result<()> {
        let items = items.iter().map(totp::SortItem::from).collect::<Vec<_>>();

        totp::sort(user_id, &items).await
    }
}

pub async fn all(access_token: &access_token::AccessToken) -> Result<Vec<DetailResponse>> {
    let totp = totp::all(access_token.user_id).await?;

    totp.into_iter().map(|t| t.try_into()).collect()
}

pub async fn sort(access_token: &access_token::AccessToken, items: Vec<SortItem>) -> Result<()> {
    sort_with_store(&DatabaseTotpSortStore, access_token.user_id, &items).await
}

async fn sort_with_store<S: TotpSortStore>(
    store: &S,
    user_id: u64,
    items: &[SortItem],
) -> Result<()> {
    let owned_ids = store.owned_ids(user_id).await?;
    let owned_id_set = owned_ids.iter().copied().collect::<BTreeSet<_>>();
    let item_id_set = items.iter().map(|item| item.id).collect::<BTreeSet<_>>();

    if owned_ids.len() != items.len()
        || item_id_set.len() != items.len()
        || owned_id_set != item_id_set
    {
        return Err(Error::AuthorizationPermissionUngranted(None));
    }

    store.replace_all_sorts(user_id, items).await
}

pub async fn detail(access_token: &access_token::AccessToken, id: u64) -> Result<DetailResponse> {
    let totp = totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp.try_into()
}

pub async fn create(
    access_token: &access_token::AccessToken,
    uri: String,
) -> Result<DetailResponse> {
    let totp = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::ParamsTotpParseFailed(None)
    })?;

    totp::insert(totp::CreatedTotp {
        user_id: access_token.user_id,
        sort: None,
        username: totp.account_name,
        issuer: totp.issuer,
        config: totp::TotpConfig {
            period: totp.step,
            secret: Secret::Raw(totp.secret).to_encoded().to_string(),
        },
    })
    .await?
    .try_into()
}

pub async fn edit_issuer(
    access_token: &access_token::AccessToken,
    params: EditIssuerRequestParams,
) -> Result<()> {
    let totp = totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp::update_issuer(totp.id, params.issuer.as_str()).await?;

    Ok(())
}

pub async fn edit_username(
    access_token: &access_token::AccessToken,
    params: EditUsernameRequestParams,
) -> Result<()> {
    let totp = totp::fetch(params.id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp::update_username(totp.id, params.username.as_str()).await?;

    Ok(())
}

pub async fn delete(access_token: &access_token::AccessToken, id: u64) -> Result<()> {
    let totp = totp::fetch(id).await?;

    totp.ensure_permission(access_token.user_id)?;

    totp::delete(id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use application_kernel::result::Error;
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    struct MemoryTotpSortStore {
        owned_ids: Vec<u64>,
        sorts: Mutex<BTreeMap<u64, u32>>,
        fail_on_id: Option<u64>,
    }

    impl MemoryTotpSortStore {
        fn new(items: &[(u64, u32)], fail_on_id: Option<u64>) -> Self {
            Self {
                owned_ids: items.iter().map(|(id, _)| *id).collect(),
                sorts: Mutex::new(items.iter().copied().collect()),
                fail_on_id,
            }
        }

        fn snapshot(&self) -> BTreeMap<u64, u32> {
            self.sorts.lock().expect("sort store poisoned").clone()
        }
    }

    impl TotpSortStore for MemoryTotpSortStore {
        async fn owned_ids(&self, _user_id: u64) -> Result<Vec<u64>> {
            Ok(self.owned_ids.clone())
        }

        async fn replace_all_sorts(&self, _user_id: u64, items: &[SortItem]) -> Result<()> {
            let current = self.snapshot();
            let mut next = current.clone();

            for item in items {
                if self.fail_on_id == Some(item.id) {
                    return Err(Error::InternalDatabaseUpdate(None));
                }

                next.insert(item.id, item.sort);
            }

            *self.sorts.lock().expect("sort store poisoned") = next;

            Ok(())
        }
    }

    #[tokio::test]
    async fn sort_updates_all_owned_items_successfully() {
        let store = MemoryTotpSortStore::new(&[(1, 3), (2, 1)], None);

        let result = sort_with_store(
            &store,
            42,
            &[SortItem { id: 1, sort: 9 }, SortItem { id: 2, sort: 4 }],
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(store.snapshot(), BTreeMap::from([(1, 9), (2, 4)]));
    }

    #[tokio::test]
    async fn sort_rejects_items_outside_current_user() {
        let store = MemoryTotpSortStore::new(&[(1, 3), (2, 1)], None);

        let result = sort_with_store(
            &store,
            42,
            &[SortItem { id: 1, sort: 9 }, SortItem { id: 3, sort: 4 }],
        )
        .await;

        assert_eq!(result, Err(Error::AuthorizationPermissionUngranted(None)));
        assert_eq!(store.snapshot(), BTreeMap::from([(1, 3), (2, 1)]));
    }

    #[tokio::test]
    async fn sort_rolls_back_when_batch_update_fails() {
        let store = MemoryTotpSortStore::new(&[(1, 3), (2, 1)], Some(2));

        let result = sort_with_store(
            &store,
            42,
            &[SortItem { id: 1, sort: 9 }, SortItem { id: 2, sort: 4 }],
        )
        .await;

        assert_eq!(result, Err(Error::InternalDatabaseUpdate(None)));
        assert_eq!(store.snapshot(), BTreeMap::from([(1, 3), (2, 1)]));
    }
}
