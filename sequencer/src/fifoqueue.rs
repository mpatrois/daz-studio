// {todo} It's not exactly the same thing but take a look at this
// absolutely wonderful ring buffer implementation:
//
// https://github.com/timjrd/wasm-spectrogram/blob/master/src/ring.rs
pub struct FifoQueue<T: Clone> {
    pub write_index: usize,
    pub read_index: usize,
    pub nb_elements: usize,
    pub queue_size: usize,
    pub fifo_queue_elements: Vec<Option<T>>
}

impl<T: Clone> FifoQueue<T> {
    pub fn new(nb_elements: usize) -> FifoQueue<T> {
        // {todo} You could have used `return` BUT YOU DIDN'T!
        // CONGRATULATIONS!
        FifoQueue { 
            write_index: 0,
            read_index: 0,
            nb_elements: nb_elements,
            queue_size: nb_elements + 1,
            fifo_queue_elements: vec![None; nb_elements + 1]
        }
    }

    pub fn write(&mut self, new_item: T) -> bool {
        if self.write_index == (( self.read_index + self.nb_elements) % self.queue_size) {
            // {todo} You can easily remove this `return` by using an
            // `else` branch, but that's OK.
            return false; /* Queue Full*/
        }
        
        self.fifo_queue_elements[self.write_index] = Some(new_item);
        self.write_index = (self.write_index + 1) % self.queue_size;

        // {todo} The `return` keyword is useless here.
        return true;
    }

    pub fn read(&mut self) -> Option<&T>
    {
        if self.write_index == self.read_index {
            // {todo} You can easily remove this `return` by using an
            // `else` branch, but that's OK.
            return None; 
        }
        let item = self.fifo_queue_elements[self.read_index].as_ref();
        self.read_index = (self.read_index + 1) % self.queue_size;
        // {todo} The `return` keyword is useless here.
        return item;
    }
}