// Interface test 18
collab Processable18 {
    slay process() drip
    slay getId() drip
}

squad Item18 {
    spill value drip
    
    slay process() drip {
        damn value * 18
    }
    
    slay getId() drip {
        damn 18
    }
}

sus item18 Item18 = Item18{value: 90}
sus processed drip = item18.process()
vibez.spill("Interface 18:", processed)
