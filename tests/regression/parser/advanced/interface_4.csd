// Interface test 4
collab Processable4 {
    slay process() drip
    slay getId() drip
}

squad Item4 {
    spill value drip
    
    slay process() drip {
        damn value * 4
    }
    
    slay getId() drip {
        damn 4
    }
}

sus item4 Item4 = Item4{value: 20}
sus processed drip = item4.process()
vibez.spill("Interface 4:", processed)
