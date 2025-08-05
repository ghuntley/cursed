squad Container[T] {
    value T
}

slay new_container[T](v T) Container[T] {
    damn Container[T]{value: v}
}

slay get_value[T](c Container[T]) T {
    damn c.value
}

slay process[T comparable](items []T) T where T: Comparable {
    sus max = items[0]
    bestie item flex items {
        lowkey item > max {
            max = item
        }
    }
    damn max
}
