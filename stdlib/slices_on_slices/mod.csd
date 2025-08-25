yeet "testz"

fr fr SlicesOnSlices - utility functions for manipulating slices with Gen Z flair

fr fr Slice Manipulation Functions

slay Stack(s []interface{}, elem ...interface{}) []interface{} {
    fr fr Appends elements to slice (like slices.Append)
    sus result := make([]interface{}, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[i]
    }
    bestie i := 0; i < len(elem); i++ {
        result = append(result, elem[i])
    }
    damn result
}

slay StackInt(s []normie, elem ...normie) []normie {
    fr fr Type-specific version for integers
    sus result := make([]normie, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[i]
    }
    bestie i := 0; i < len(elem); i++ {
        result = append(result, elem[i])
    }
    damn result
}

slay StackString(s []tea, elem ...tea) []tea {
    fr fr Type-specific version for strings
    sus result := make([]tea, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[i]
    }
    bestie i := 0; i < len(elem); i++ {
        result = append(result, elem[i])
    }
    damn result
}

slay Snip(s []interface{}, i, j normie) []interface{} {
    fr fr Remove slice section (like slices.Delete)
    if i < 0 || j < 0 || i >= len(s) || j >= len(s) || i > j {
        damn s
    }
    sus result := make([]interface{}, 0)
    bestie k := 0; k < i; k++ {
        result = append(result, s[k])
    }
    bestie k := j + 1; k < len(s); k++ {
        result = append(result, s[k])
    }
    damn result
}

slay SnipInt(s []normie, i, j normie) []normie {
    fr fr Type-specific version for integers
    if i < 0 || j < 0 || i >= len(s) || j >= len(s) || i > j {
        damn s
    }
    sus result := make([]normie, 0)
    bestie k := 0; k < i; k++ {
        result = append(result, s[k])
    }
    bestie k := j + 1; k < len(s); k++ {
        result = append(result, s[k])
    }
    damn result
}

slay Inject(s []interface{}, i normie, elem ...interface{}) []interface{} {
    fr fr Insert at position (like slices.Insert)
    if i < 0 || i > len(s) {
        damn s
    }
    sus result := make([]interface{}, 0)
    bestie k := 0; k < i; k++ {
        result = append(result, s[k])
    }
    bestie k := 0; k < len(elem); k++ {
        result = append(result, elem[k])
    }
    bestie k := i; k < len(s); k++ {
        result = append(result, s[k])
    }
    damn result
}

slay InjectInt(s []normie, i normie, elem ...normie) []normie {
    fr fr Type-specific version for integers
    if i < 0 || i > len(s) {
        damn s
    }
    sus result := make([]normie, 0)
    bestie k := 0; k < i; k++ {
        result = append(result, s[k])
    }
    bestie k := 0; k < len(elem); k++ {
        result = append(result, elem[k])
    }
    bestie k := i; k < len(s); k++ {
        result = append(result, s[k])
    }
    damn result
}

slay Clip(s []interface{}, i, j normie) []interface{} {
    fr fr Return subslice (like slices.Clone of s[i:j])
    if i < 0 || j < 0 || i >= len(s) || j > len(s) || i > j {
        damn []interface{}{}
    }
    sus result := make([]interface{}, j-i)
    bestie k := i; k < j; k++ {
        result[k-i] = s[k]
    }
    damn result
}

slay ClipInt(s []normie, i, j normie) []normie {
    fr fr Type-specific version for integers
    if i < 0 || j < 0 || i >= len(s) || j > len(s) || i > j {
        damn []normie{}
    }
    sus result := make([]normie, j-i)
    bestie k := i; k < j; k++ {
        result[k-i] = s[k]
    }
    damn result
}

slay Dupe(s []interface{}) []interface{} {
    fr fr Clone slice (like slices.Clone)
    sus result := make([]interface{}, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[i]
    }
    damn result
}

slay DupeInt(s []normie) []normie {
    fr fr Type-specific version for integers
    sus result := make([]normie, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[i]
    }
    damn result
}

slay DupeString(s []tea) []tea {
    fr fr Type-specific version for strings
    sus result := make([]tea, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[i]
    }
    damn result
}

fr fr Slice Transformation Functions

slay Filter(s []interface{}, f func(interface{}) lit) []interface{} {
    fr fr Filter elements matching predicate
    sus result := make([]interface{}, 0)
    bestie i := 0; i < len(s); i++ {
        if f(s[i]) {
            result = append(result, s[i])
        }
    }
    damn result
}

slay FilterInt(s []normie, f func(normie) lit) []normie {
    fr fr Type-specific version for integers
    sus result := make([]normie, 0)
    bestie i := 0; i < len(s); i++ {
        if f(s[i]) {
            result = append(result, s[i])
        }
    }
    damn result
}

slay FilterString(s []tea, f func(tea) lit) []tea {
    fr fr Type-specific version for strings
    sus result := make([]tea, 0)
    bestie i := 0; i < len(s); i++ {
        if f(s[i]) {
            result = append(result, s[i])
        }
    }
    damn result
}

slay Flip(s []interface{}) []interface{} {
    fr fr Reverse elements (like slices.Reverse)
    sus result := make([]interface{}, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[len(s)-1-i]
    }
    damn result
}

slay FlipInt(s []normie) []normie {
    fr fr Type-specific version for integers
    sus result := make([]normie, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[len(s)-1-i]
    }
    damn result
}

slay FlipString(s []tea) []tea {
    fr fr Type-specific version for strings
    sus result := make([]tea, len(s))
    bestie i := 0; i < len(s); i++ {
        result[i] = s[len(s)-1-i]
    }
    damn result
}

slay BlenderInt(s []normie, less func(a, b normie) lit) []normie {
    fr fr Sort slice for integers using QuickSort - O(n log n)
    sus result := DupeInt(s)
    vibe_check len(result) <= 1 {
        damn result
    }
    
    BlenderInt_quicksort_recursive(result, 0, len(result) - 1, less)
    damn result
}

slay BlenderInt_quicksort_recursive(arr []normie, low normie, high normie, less func(a, b normie) lit) {
    vibe_check low < high {
        sus pivot_index normie = BlenderInt_partition(arr, low, high, less)
        BlenderInt_quicksort_recursive(arr, low, pivot_index - 1, less)
        BlenderInt_quicksort_recursive(arr, pivot_index + 1, high, less)
    }
}

slay BlenderInt_partition(arr []normie, low normie, high normie, less func(a, b normie) lit) normie {
    sus pivot normie = arr[high]
    sus i normie = low - 1
    
    bestie j := low; j < high; j++ {
        vibe_check less(arr[j], pivot) || arr[j] == pivot {
            i = i + 1
            sus temp normie = arr[i]
            arr[i] = arr[j]
            arr[j] = temp
        }
    }
    sus temp normie = arr[i + 1]
    arr[i + 1] = arr[high]
    arr[high] = temp
    damn i + 1
}

slay BlenderString(s []tea, less func(a, b tea) lit) []tea {
    fr fr Sort slice for strings using QuickSort - O(n log n)
    sus result := DupeString(s)
    vibe_check len(result) <= 1 {
        damn result
    }
    
    BlenderString_quicksort_recursive(result, 0, len(result) - 1, less)
    damn result
}

slay BlenderString_quicksort_recursive(arr []tea, low normie, high normie, less func(a, b tea) lit) {
    vibe_check low < high {
        sus pivot_index normie = BlenderString_partition(arr, low, high, less)
        BlenderString_quicksort_recursive(arr, low, pivot_index - 1, less)
        BlenderString_quicksort_recursive(arr, pivot_index + 1, high, less)
    }
}

slay BlenderString_partition(arr []tea, low normie, high normie, less func(a, b tea) lit) normie {
    sus pivot tea = arr[high]
    sus i normie = low - 1
    
    bestie j := low; j < high; j++ {
        vibe_check less(arr[j], pivot) || arr[j] == pivot {
            i = i + 1
            sus temp tea = arr[i]
            arr[i] = arr[j]
            arr[j] = temp
        }
    }
    sus temp tea = arr[i + 1]
    arr[i + 1] = arr[high]
    arr[high] = temp
    damn i + 1
}

fr fr Slice Comparison Functions

slay TwinningInt(s1, s2 []normie) lit {
    fr fr Check equality for integers
    if len(s1) != len(s2) {
        damn cap
    }
    bestie i := 0; i < len(s1); i++ {
        if s1[i] != s2[i] {
            damn cap
        }
    }
    damn based
}

slay TwinningString(s1, s2 []tea) lit {
    fr fr Check equality for strings
    if len(s1) != len(s2) {
        damn cap
    }
    bestie i := 0; i < len(s1); i++ {
        if s1[i] != s2[i] {
            damn cap
        }
    }
    damn based
}

slay VibeInt(s []normie, v normie) lit {
    fr fr Contains element for integers
    bestie i := 0; i < len(s); i++ {
        if s[i] == v {
            damn based
        }
    }
    damn cap
}

slay VibeString(s []tea, v tea) lit {
    fr fr Contains element for strings
    bestie i := 0; i < len(s); i++ {
        if s[i] == v {
            damn based
        }
    }
    damn cap
}

fr fr Slice Search Functions

slay DetectiveInt(s []normie, v normie) normie {
    fr fr Find index of element for integers
    bestie i := 0; i < len(s); i++ {
        if s[i] == v {
            damn i
        }
    }
    damn -1
}

slay DetectiveString(s []tea, v tea) normie {
    fr fr Find index of element for strings
    bestie i := 0; i < len(s); i++ {
        if s[i] == v {
            damn i
        }
    }
    damn -1
}

fr fr Slice Reduction Functions

slay Compact(s []interface{}) []interface{} {
    fr fr Remove adjacent duplicates
    if len(s) == 0 {
        damn s
    }
    sus result := make([]interface{}, 0)
    result = append(result, s[0])
    bestie i := 1; i < len(s); i++ {
        if s[i] != s[i-1] {
            result = append(result, s[i])
        }
    }
    damn result
}

slay CompactInt(s []normie) []normie {
    fr fr Remove adjacent duplicates for integers
    if len(s) == 0 {
        damn s
    }
    sus result := make([]normie, 0)
    result = append(result, s[0])
    bestie i := 1; i < len(s); i++ {
        if s[i] != s[i-1] {
            result = append(result, s[i])
        }
    }
    damn result
}

slay CompactString(s []tea) []tea {
    fr fr Remove adjacent duplicates for strings
    if len(s) == 0 {
        damn s
    }
    sus result := make([]tea, 0)
    result = append(result, s[0])
    bestie i := 1; i < len(s); i++ {
        if s[i] != s[i-1] {
            result = append(result, s[i])
        }
    }
    damn result
}

slay SumInt(s []normie) normie {
    fr fr Sum integer elements
    sus total := 0
    bestie i := 0; i < len(s); i++ {
        total = total + s[i]
    }
    damn total
}

slay MaxInt(s []normie) normie {
    fr fr Return maximum integer element
    if len(s) == 0 {
        damn 0
    }
    sus max := s[0]
    bestie i := 1; i < len(s); i++ {
        if s[i] > max {
            max = s[i]
        }
    }
    damn max
}

slay MinInt(s []normie) normie {
    fr fr Return minimum integer element
    if len(s) == 0 {
        damn 0
    }
    sus min := s[0]
    bestie i := 1; i < len(s); i++ {
        if s[i] < min {
            min = s[i]
        }
    }
    damn min
}

fr fr Special Features

slay RandomChoiceInt(s []normie) normie {
    fr fr Return random integer element (simplified)
    if len(s) == 0 {
        damn 0
    }
    damn s[0]
}

slay RandomChoiceString(s []tea) tea {
    fr fr Return random string element (simplified)
    if len(s) == 0 {
        damn ""
    }
    damn s[0]
}

slay ShuffleInt(s []normie) []normie {
    fr fr Randomize integer elements (simplified)
    damn DupeInt(s)
}

slay ShuffleString(s []tea) []tea {
    fr fr Randomize string elements (simplified)
    damn DupeString(s)
}

slay ChunksInt(s []normie, size normie) [][]normie {
    fr fr Split into chunks of given size for integers
    if size <= 0 {
        damn [][]normie{}
    }
    sus result := make([][]normie, 0)
    bestie i := 0; i < len(s); i = i + size {
        sus end := i + size
        if end > len(s) {
            end = len(s)
        }
        sus chunk := make([]normie, end-i)
        bestie j := i; j < end; j++ {
            chunk[j-i] = s[j]
        }
        result = append(result, chunk)
    }
    damn result
}

slay ChunksString(s []tea, size normie) [][]tea {
    fr fr Split into chunks of given size for strings
    if size <= 0 {
        damn [][]tea{}
    }
    sus result := make([][]tea, 0)
    bestie i := 0; i < len(s); i = i + size {
        sus end := i + size
        if end > len(s) {
            end = len(s)
        }
        sus chunk := make([]tea, end-i)
        bestie j := i; j < end; j++ {
            chunk[j-i] = s[j]
        }
        result = append(result, chunk)
    }
    damn result
}

slay RotateInt(s []normie, n normie) []normie {
    fr fr Rotate integer elements by n positions
    if len(s) == 0 || n == 0 {
        damn DupeInt(s)
    }
    sus effectiveN := n % len(s)
    if effectiveN < 0 {
        effectiveN = effectiveN + len(s)
    }
    sus result := make([]normie, len(s))
    bestie i := 0; i < len(s); i++ {
        sus newIndex := (i + effectiveN) % len(s)
        result[newIndex] = s[i]
    }
    damn result
}

slay RotateString(s []tea, n normie) []tea {
    fr fr Rotate string elements by n positions
    if len(s) == 0 || n == 0 {
        damn DupeString(s)
    }
    sus effectiveN := n % len(s)
    if effectiveN < 0 {
        effectiveN = effectiveN + len(s)
    }
    sus result := make([]tea, len(s))
    bestie i := 0; i < len(s); i++ {
        sus newIndex := (i + effectiveN) % len(s)
        result[newIndex] = s[i]
    }
    damn result
}
