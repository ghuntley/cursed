slay check_age(age normie) {
    lowkey age >= 18 {
        print("Adult")
    } highkey lowkey age >= 13 {
        print("Teenager")
    } highkey {
        print("Child")
    }
}

slay simple_check(x normie) {
    lowkey (x > 0) {
        print("Positive")
    }
}