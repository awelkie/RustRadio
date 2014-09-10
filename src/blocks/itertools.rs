//TODO make this use unboxed closures once they're in the prelude
pub struct MapChunk2<'a, A, B, It> {
    iter: It,
    f: |[A, ..2]|: 'a -> B,
}

impl<'a, A, B, It: Iterator<A>> Iterator<B> for MapChunk2<'a, A, B, It> {
    #[inline]
    fn next(&mut self) -> Option<B> {
        let a = self.iter.next();
        let b = self.iter.next();
        match (a,b) {
            (Some(x), Some(y)) => Some((self.f)([x,y])),
            _ => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        self.iter.size_hint()
    }
}

pub fn map_chunk_2<'r, A, B, I: Iterator<A>>(iter: I, f: |[A, ..2]|: 'r -> B) -> MapChunk2<'r, A, B, I> {
    MapChunk2 { iter: iter, f: f }
}
