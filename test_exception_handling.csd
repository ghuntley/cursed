slay main() {
    catch {
        vibez.spill("About to cause an error");
        yeet_error("Something went wrong!");
    } recovery {
        vibez.spill("Caught the error and recovered!");
    }
    vibez.spill("Program completed successfully");
    yolo 0;
}
