// Interface test 1
collab Processable1 {
    slay process() drip
    slay getId() drip
}

squad Item1 {
    spill value drip
    
    slay process() drip {
        damn value * 1
    }
    
    slay getId() drip {
        damn 1
    }
}

sus item1 Item1 = Item1{value: 5}
sus processed drip = item1.process()
vibez.spill("Interface 1:", processed)
