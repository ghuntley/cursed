fr fr Test file for pointer functionality

slay main() {
    fr fr Test basic pointer creation and dereferencing
    sus x normie = 42;
    sus ptr = @x;
    sus y = @ptr;
    
    fr fr Test that y equals x via the pointer
    lowkey y == 42 {
        puts(1);
        damn 0;
    } highkey {
        puts(0);
        damn 1;
    }
}

be_like Person squad {
    name tea;
    age normie;
}