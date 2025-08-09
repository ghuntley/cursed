// Interface test 9
collab Processable9 {
    slay process() drip
    slay getId() drip
}

squad Item9 {
    spill value drip
    
    slay process() drip {
        damn value * 9
    }
    
    slay getId() drip {
        damn 9
    }
}

sus item9 Item9 = Item9{value: 45}
sus processed drip = item9.process()
vibez.spill("Interface 9:", processed)
