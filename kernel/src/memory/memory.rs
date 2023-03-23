pub struct Block {
    start: usize,
    size: usize,
    free: bool,
}

pub struct BuddyAllocator {
    memory: *mut u8,
    size: usize,
    min_block_size: usize,
    blocks: Vec<Vec<Block>>,
}

impl BuddyAllocator {
    pub fn new(memory: *mut u8, size: usize, min_block_size: usize) -> Self {
        let mut blocks = Vec::new();
        let levels = (size / min_block_size).next_power_of_two().trailing_zeros() as usize;

        for _ in 0..=levels {
            blocks.push(Vec::new());
        }

        blocks[levels].push(Block {
            start: 0,
            size: size,
            free: true,
        });

        BuddyAllocator {
            memory,
            size,
            min_block_size,
            blocks,
        }
    }

    fn index_of_buddy(&self, index: usize, level: usize) -> usize {
        index ^ (1 << level)
    }

    fn split_block(&mut self, level: usize) {
        let block = self.blocks[level].pop().unwrap();

        let left_block = Block {
            start: block.start,
            size: block.size / 2,
            free: true,
        };

        let right_block = Block {
            start: block.start + left_block.size,
            size: block.size / 2,
            free: true,
        };

        self.blocks[level - 1].push(left_block);
        self.blocks[level - 1].push(right_block);
    }

    fn merge_blocks(&mut self, index: usize, level: usize) {
        let buddy_index = self.index_of_buddy(index, level);
        let buddy = self.blocks[level][buddy_index];

        if buddy.free {
            self.blocks[level].remove(buddy_index);
            self.blocks[level].remove(index);
            self.blocks[level + 1].push(Block {
                start: buddy.start.min(self.blocks[level][index].start),
                size: buddy.size * 2,
                free: true,
            });
        }
    }

    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        let level = (((size + self.min_block_size - 1) / self.min_block_size)
            .next_power_of_two()
            .trailing_zeros() as usize)
            .max(1);

        for i in level..self.blocks.len() {
            if !self.blocks[i].is_empty() {
                let mut index = None;

                for (j, block) in self.blocks[i].iter().enumerate() {
                    if block.free {
                        index = Some(j);
                        break;
                    }
                }

                if let Some(index) = index {
                    self.blocks[i][index].free = false;

                    while i > level {
                        self.split_block(i);
                    }

                    return unsafe { self.memory.offset(self.blocks[level][index].start as isize) };
                }
            }
        }

        std::ptr::null_mut()
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        let level = (((size + self.min_block_size - 1) / self.min_block_size)
            .next_power_of_two()
            .trailing_zeros() as usize)
            .max(1);

        let start = unsafe { (ptr as usize) - (self.memory as usize) };

        let mut index = None;
        for (i, block) in self.blocks[level].iter().enumerate() {
            if block.start == start {
                index = Some(i);
                break;
            }
        }

        if let Some(index) = index {
            self.blocks[level][index].free = true;

            let mut current_level = level;
            while current_level < self.blocks.len() - 1 {
                if self.blocks[current_level].len() % 2 == 0 {
                    let buddy_index = self.index_of_buddy(index, current_level);
                    if self.blocks[current_level][buddy_index].free {
                        self.merge_blocks(index, current_level);
                        current_level += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}