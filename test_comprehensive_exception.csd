fn main() {
    catch {
        yeet_error("Test exception message");
    } recover (error) {
        // This should be handled when the exception is caught
    }
}
