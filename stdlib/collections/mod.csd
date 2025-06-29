// Standard collections library
fn array_new() -> array {
    return [];
}

fn array_push(arr: array, item: any) -> array {
    arr.push(item);
    return arr;
}

fn map_new() -> map {
    return {};
}

fn map_set(m: map, key: string, value: any) -> map {
    m[key] = value;
    return m;
}

fn map_get(m: map, key: string) -> any {
    return m[key];
}
