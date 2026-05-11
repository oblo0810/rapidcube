use super::Cube3x3;

impl Cube3x3 {
    fn to_hash_edge(&self) -> i64 {
        let mut hash: i64 = 0; 
        for i in 0..11 {
            let (_, ori) = self.get_edge_internal(i);
            hash |= (ori as i64) << i;
        }
        hash
    }

    fn to_hash_corner(&self) -> i64 {
        let mut hash: i64 = 0;
        let mut multiplier: i64 = 1;
        for i in 0..7 {
            let (_, ori) = self.get_corner_internal(i);
            hash += (ori as i64) * multiplier;
            multiplier *= 3; 
        }
        hash
    }

    // fn to_hash_ud(&self) -> i64 {
    //     let mut hash: i64 = 0;
    //     let ud_idx = [8, 9, 10, 11];
    //     for i in 0..12 {
    //         let (pos, _) = self.get_edge_internal(i);
    //         if ud_idx.contains(&(pos as usize)) {
    //             hash += i choose k; //TODO
    //         }
    //     }
    //     hash
    // }
}

// LD_LIBRARY_PATH=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))") cargo test

#[cfg(test)]
mod tests {
    use super::*;

    fn make_edges_from_oris(oris: [u8; 11]) -> u64 {
        let mut edges: u64 = 0;
        for (i, &ori) in oris.iter().enumerate() {
            let pos: u64 = 0;
            let data = pos | ((ori as u64) << 4);
            let shift = i * 5;
            edges |= data << shift;
        }
        edges
    }

    #[test]
    fn to_hash_edge_pattern() {
        let oris = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let edges = make_edges_from_oris(oris);
        let c = Cube3x3 { corners: 0, edges };
        let mut expected: i64 = 0;
        for (i, &o) in oris.iter().enumerate() {
            expected |= (o as i64) << i;
        }
        assert_eq!(c.to_hash_edge(), expected);
    }
}


