vibez.spill("Testing bestie logic...")

sus a normie = 5
sus b normie = 3
sus result normie = 0

bestie a > b {
    result = a
    vibez.spill("a is greater: " + tea(a))
}

bestie b > a {
    result = b 
    vibez.spill("b is greater: " + tea(b))
}

bestie a == b {
    result = a
    vibez.spill("Equal values: " + tea(a))
}

vibez.spill("Result: " + tea(result))
