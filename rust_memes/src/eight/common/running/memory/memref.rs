#[derive(Debug)]
pub enum MemData {
    StackMem(i32),
    FrameMem(String),
    EmptyMem
}

#[derive(Debug)]
pub struct MemRef {
    data: MemData
}

impl MemRef {
    pub fn empty() -> Self {
        MemRef{
            data: MemData::EmptyMem
        }
    }

    pub fn copy(&self) -> Self {
        // Create a copy of the mem reference
        let md: MemData = match &self.data {
            MemData::FrameMem(s) => MemData::FrameMem(s.clone()),
            MemData::StackMem(i) => MemData::StackMem(i.clone()),
            MemData::EmptyMem => MemData::EmptyMem
        };

        MemRef {
            data: md
        }
    }

    pub fn is_stack_ref(&self) -> bool {
        match &self.data {
            MemData::StackMem(_) => true,
            _ => false
        }
    }

    pub fn is_frame_ref(&self) -> bool {
        match &self.data {
            MemData::FrameMem(_) => true,
            _ => false
        }
    }

    pub fn get_as_stack(&self) -> &i32 {
        return match &self.data {
            MemData::StackMem(idx) => idx,
            MemData::FrameMem(f) => {
                // TODO Runtime exceptions
                panic!("Shouldn't be here! Attempted stack mem access with a frame mem reference '{:?}'", f);
            },
            MemData::EmptyMem => {
                // TODO Runtime exceptions
                panic!("Attempted stack mem access of empty memory!");
            }
        }
    }

    pub fn get_as_frame(&self) -> &str {
        return match &self.data {
            MemData::FrameMem(f) => f,
            MemData::StackMem(s) => {
                // TODO Runtime exceptions
                panic!("Shouldn't be here! Attempted frame mem access with a stack mem reference '{:?}'", s);
            },
            MemData::EmptyMem => {
                // TODO Runtime exceptions
                panic!("Attempted stack mem access of empty memory!");
            }
        }
    }

    pub fn stack(idx: i32) -> Self {
        MemRef {
            data: MemData::StackMem(idx)
        }
    }

    pub fn frame(name: String) -> Self {
        MemRef {
            data: MemData::FrameMem(name)
        }
    }
}
