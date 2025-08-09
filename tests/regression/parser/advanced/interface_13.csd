// Interface test 13
collab Processable13 {
    slay process() drip
    slay getId() drip
}

squad Item13 {
    spill value drip
    
    slay process() drip {
        damn value * 13
    }
    
    slay getId() drip {
        damn 13
    }
}

sus item13 Item13 = Item13{value: 65}
sus processed drip = item13.process()
vibez.spill("Interface 13:", processed)
