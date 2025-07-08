// Pure CURSED List Implementation - Working version

slay list_new() {
    damn [];
}

slay list_add(lst, value) {
    damn append(lst, value);
}

slay list_get(lst, index) {
    damn lst[index];
}

slay list_size(lst) {
    damn len(lst);
}

slay list_is_empty(lst) {
    damn len(lst) == 0;
}

slay list_print(lst) {
    vibez.spill("List contents:");
    vibez.spill(lst);
}
