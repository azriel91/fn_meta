
impl<Fun, Ret, A0> FnMetadata<Fun, Ret, (&A0,)>
where
    Fun: FnOnce(&A0,) -> Ret,
    A0: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0> FnMetadata<Fun, Ret, (&mut A0,)>
where
    Fun: FnOnce(&mut A0,) -> Ret,
    A0: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1> FnMetadata<Fun, Ret, (&A0, &A1)>
where
    Fun: FnOnce(&A0, &A1) -> Ret,
    A0: 'static,
    A1: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0, A1> FnMetadata<Fun, Ret, (&mut A0, &A1)>
where
    Fun: FnOnce(&mut A0, &A1) -> Ret,
    A0: 'static,
    A1: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1> FnMetadata<Fun, Ret, (&A0, &mut A1)>
where
    Fun: FnOnce(&A0, &mut A1) -> Ret,
    A0: 'static,
    A1: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1> FnMetadata<Fun, Ret, (&mut A0, &mut A1)>
where
    Fun: FnOnce(&mut A0, &mut A1) -> Ret,
    A0: 'static,
    A1: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&A0, &A1, &A2)>
where
    Fun: FnOnce(&A0, &A1, &A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2)>
where
    Fun: FnOnce(&mut A0, &A1, &A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2)>
where
    Fun: FnOnce(&A0, &mut A1, &A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2)>
where
    Fun: FnOnce(&A0, &A1, &mut A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 7] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 0] {
        []
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A5>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A4>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A6>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 1] {
        [TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A4>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A5>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 2] {
        [TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A3>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A4>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 3] {
        [TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A0>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A3>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 3] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 4] {
        [TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A0>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A2>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 2] {
        [TypeId::of::<A0>(), TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 5] {
        [TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A1>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A0>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 1] {
        [TypeId::of::<A0>()]
    }

    pub fn writes(&self) -> [TypeId; 6] {
        [TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    
impl<Fun, Ret, A0, A1, A2, A3, A4, A5, A6> FnMetadata<Fun, Ret, (&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6)>
where
    Fun: FnOnce(&mut A0, &mut A1, &mut A2, &mut A3, &mut A4, &mut A5, &mut A6) -> Ret,
    A0: 'static,
    A1: 'static,
    A2: 'static,
    A3: 'static,
    A4: 'static,
    A5: 'static,
    A6: 'static,
{
    pub fn reads(&self) -> [TypeId; 0] {
        []
    }

    pub fn writes(&self) -> [TypeId; 7] {
        [TypeId::of::<A0>(), TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>(), TypeId::of::<A4>(), TypeId::of::<A5>(), TypeId::of::<A6>()]
    }
}
    