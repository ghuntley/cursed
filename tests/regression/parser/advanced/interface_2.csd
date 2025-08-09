// Interface test 2
collab Processable2 {
    slay process() drip
    slay getId() drip
}

squad Item2 {
    spill value drip
    
    slay process() drip {
        damn value * 2
    }
    
    slay getId() drip {
        damn 2
    }
}

sus item2 Item2 = Item2{value: 10}
sus processed drip = item2.process()
vibez.spill("Interface 2:", processed)
