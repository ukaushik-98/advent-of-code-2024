use tokio::fs;

async fn readFile() {
    let content = fs::read("sample.txt");
}
