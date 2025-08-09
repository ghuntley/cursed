// Interface test 3
collab Processable3 {
    slay process() drip
    slay getId() drip
}

squad Item3 {
    spill value drip
    
    slay process() drip {
        damn value * 3
    }
    
    slay getId() drip {
        damn 3
    }
}

sus item3 Item3 = Item3{value: 15}
sus processed drip = item3.process()
vibez.spill("Interface 3:", processed)
