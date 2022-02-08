const ARRAY_SIZE: usize = 10_000;

pub struct InliningExperiment
{
    array: [u16; ARRAY_SIZE],
}

//Public Functions
impl InliningExperiment
{
    #[inline(always)]
    pub fn new() -> InliningExperiment
    {
        return InliningExperiment { array: InliningExperiment::generate_array_of_ordered_ints() };
    }

    #[inline(always)]
    pub fn get_array(&self) -> [u16; ARRAY_SIZE]
    {
        return self.array;
    }

    #[inline(always)]
    pub fn flip_array(&mut self, inlined: bool)
    {
        if inlined
        {
            self.inlined_flip_array();
        }
        else
        {
            self.outlined_flip_array();
        }
    }
}

//Private Functions
impl InliningExperiment
{
    #[inline(always)]
    fn generate_array_of_ordered_ints() -> [u16; ARRAY_SIZE]
    {
        let mut array_to_return: [u16; ARRAY_SIZE] = [0; ARRAY_SIZE];

        for counter in 0..ARRAY_SIZE
        {
            array_to_return[counter] = counter as u16;
        }

        return array_to_return;
    }

    #[inline(always)]
    fn flip_entry_at_index(&mut self, index: usize)
    {
        let temporary_storage: u16;
        let flipped_index: usize = ARRAY_SIZE - index - 1;

        temporary_storage = self.array[index];
        self.array[index] = self.array[flipped_index];
        self.array[flipped_index] = temporary_storage;
    }

    #[inline(never)]
    fn outlined_flip_array(&mut self)
    {
        self.inlined_flip_array();
    }

    #[inline(always)]
    fn inlined_flip_array(&mut self)
    {
        let half_array_size = ARRAY_SIZE / 2;

        for counter in 0..half_array_size
        {
            self.flip_entry_at_index(counter);
        }
    }
}