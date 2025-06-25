vibe test

be_like Box[T] squad {
    value T
}

slay identity[T](x T) T {
    yolo x
}

slay apply[A, B](f stan(A) B, x A) B {
    yolo f(x)
}

slay add(x normie, y normie) normie {
    yolo x + y
}

slay main() normie {
    sus box_int = Box[normie]{value: 42}
    sus int_result = identity[normie](box_int.value)
    
    sus box_string = Box[tea]{value: "hello"}
    sus string_result = identity[tea](box_string.value)
    
    sus addition_result = apply[normie, normie](add, 5, 3)
    
    yolo addition_result
}