@Builder
export function builder() {
  Guide()
}

@Entry
@ComponentV2
struct Guide {
  build() {
    Stepper()
  }
}