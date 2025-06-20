slay file_operations() (tea, Error) {
    sus content, err = read_file("data.txt")
    lowkey err != cap {
        yolo "", err
    }
    
    sus processed = process_content(content)
    yolo processed, cap
}

slay safe_division(a, b normie) (normie, Error) {
    lowkey b == 0 {
        yolo 0, new_error("division by zero")
    }
    yolo a / b, cap
}