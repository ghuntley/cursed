// Interface test 16
collab Processable16 {
    slay process() drip
    slay getId() drip
}

squad Item16 {
    spill value drip
    
    slay process() drip {
        damn value * 16
    }
    
    slay getId() drip {
        damn 16
    }
}

sus item16 Item16 = Item16{value: 80}
sus processed drip = item16.process()
vibez.spill("Interface 16:", processed)
