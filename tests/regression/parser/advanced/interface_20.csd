// Interface test 20
collab Processable20 {
    slay process() drip
    slay getId() drip
}

squad Item20 {
    spill value drip
    
    slay process() drip {
        damn value * 20
    }
    
    slay getId() drip {
        damn 20
    }
}

sus item20 Item20 = Item20{value: 100}
sus processed drip = item20.process()
vibez.spill("Interface 20:", processed)
