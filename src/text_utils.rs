pub fn split_into_chunks(text: &str, max_length: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    // Split the input by words
    let words = text.split_whitespace();

    for word in words {
        // Check if adding the next word would exceed the max length
        if current_chunk.len() + word.len() + 1 <= max_length {
            if !current_chunk.is_empty() {
                current_chunk.push(' ');
            }
            current_chunk.push_str(word);
        } else {
            // Push the current chunk and start a new one
            chunks.push(current_chunk.clone());
            current_chunk = word.to_string(); // Start with the current word
        }
    }

    // Don't forget to push the last chunk
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}