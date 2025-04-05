vibe test

be_like Box[T] squad {
    value T
}

be_like Pair[A, B] squad {
    first A
    second B
}

slay map[T, U](input T, fn stan(T) U) U {
    yolo fn(input)
}

slay identity[T](x T) T {
    yolo x
}

slay main() {
    sus box = Box[normie]{value: 42}
    sus pair = Pair[tea, normie]{first: "hello", second: 42}
    sus result = map[normie, tea](42, stan(x) {
        yolo x.to_string()
    })
}