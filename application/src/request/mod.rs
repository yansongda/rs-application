pub mod api;

use application_kernel::result::Result;

/// todo: 感觉这里可以用一个宏来实现。比如 空检查、长度检查、类型检查等
pub trait Validator {
    type Data;

    fn validate(&self) -> Result<Self::Data>;
}
