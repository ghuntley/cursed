vibe test

be_like Box[T] squad {
    value T
}

be_like Pair[A, B] squad {
    first A
    second B
}

slay transform[K, V](collection K, mapper stan(K) V) V {
    damn mapper(collection)
}

slay main() {
    sus box_int = Box[normie]{value: 42}
    sus box_string = Box[tea]{value: "hello"}
    
    sus pair = Pair[tea, normie]{first: "world", second: 123}
    
    sus pair_of_boxes = Pair[Box[normie], Box[tea]]{
        first: Box[normie]{value: 99},
        second: Box[tea]{value: "nested"}
    }
}