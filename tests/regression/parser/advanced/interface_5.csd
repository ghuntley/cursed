// Interface test 5
collab Processable5 {
    slay process() drip
    slay getId() drip
}

squad Item5 {
    spill value drip
    
    slay process() drip {
        damn value * 5
    }
    
    slay getId() drip {
        damn 5
    }
}

sus item5 Item5 = Item5{value: 25}
sus processed drip = item5.process()
vibez.spill("Interface 5:", processed)
