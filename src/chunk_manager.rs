use std::collections::HashMap;

/*
Responsibilities:
1. Hold chunks
2. Decide to allocate chunks
3. Decide to deallocate chunks
4. Sort them for rendering?

*/
pub struct ChunkManager {
    chunk_map: HashMap<(i32,i32,i32), Chunk>;
}

impl ChunkManager {
    
}