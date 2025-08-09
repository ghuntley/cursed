// Integration test 10
yeet "mathz"
yeet "stringz"
yeet "arrayz"

squad DataProcessor10 {
    spill data []drip
    spill name tea
    
    slay process() []drip {
        sus result []drip = []
        sus idx drip = 0
        bestie (idx < len(data)) {
            sus processed drip = abs_normie(data[idx]) * 10
            result = append_drip(result, processed)
            idx = idx + 1
        }
        damn result
    }
    
    slay summary() tea {
        sus total drip = sum_drip(data)
        damn name + ": total=" + to_str_drip(total)
    }
}

slay main() {
    sus processors []DataProcessor10 = [
        DataProcessor10{data: [1,2,3,4,5], name: "processor_1"},
        DataProcessor10{data: [-3,-2,-1,0,1,2], name: "processor_2"}
    ]
    
    sus idx drip = 0
    bestie (idx < len(processors)) {
        sus processed []drip = processors[idx].process()
        sus summary tea = processors[idx].summary()
        vibez.spill("Integration 10:", summary, len(processed))
        idx = idx + 1
    }
}

main()
