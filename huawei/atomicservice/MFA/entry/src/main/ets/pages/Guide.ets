@Builder
export function builder() {
  Guide()
}

@ComponentV2
struct Guide {
  build() {
    Stepper()
  }
}