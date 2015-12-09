///! Triplet format matrix
///! Useful for building a matrix, but not for computations

/// Triplet matrix
pub struct TripletMat<N> {
    rows: usize,
    cols: usize,
    row_inds: Vec<usize>,
    col_inds: Vec<usize>,
    data: Vec<N>,
}

impl<N> TripletMat<N> {

    pub fn new(shape: (usize, usize)) -> TripletMat<N> {
        TripletMat {
            rows: shape.0,
            cols: shape.1,
            row_inds: Vec::new(),
            col_inds: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn with_capacity(shape: (usize, usize),
                         cap: usize)
                         -> TripletMat<N> {
        TripletMat {
            rows: shape.0,
            cols: shape.1,
            row_inds: Vec::with_capacity(cap),
            col_inds: Vec::with_capacity(cap),
            data: Vec::with_capacity(cap),
        }
    }

    pub fn from_triplets(shape: (usize, usize),
                         row_inds: Vec<usize>,
                         col_inds: Vec<usize>,
                         data: Vec<N>)
                         -> TripletMat<N> {
        assert!(row_inds.len() == col_inds.len(),
                "all inputs should have the same length");
        assert!(data.len() == col_inds.len(),
                "all inputs should have the same length");
        assert!(row_inds.len() == data.len(),
                "all inputs should have the same length");
        TripletMat {
            rows: shape.0,
            cols: shape.1,
            row_inds: row_inds,
            col_inds: col_inds,
            data: data,
        }
    }

    pub fn rows(&self) -> usize {
        self.borrowed().rows()
    }

    pub fn cols(&self) -> usize {
        self.borrowed().cols()
    }

    pub fn shape(&self) -> (usize, usize) {
        self.borrowed().shape()
    }

    pub fn nnz(&self) -> usize {
        self.borrowed().nnz()
    }

    pub fn row_inds(&self) -> &[usize] {
        self.borrowed().row_inds()
    }

    pub fn col_inds(&self) -> &[usize] {
        self.borrowed().col_inds()
    }

    pub fn data(&self) -> &[N] {
        self.borrowed().data()
    }

    pub fn borrowed(&self) -> TripletView<N> {
        TripletView {
            rows: self.rows,
            cols: self.cols,
            row_inds: &self.row_inds[..],
            col_inds: &self.col_inds[..],
            data: &self.data[..],
        }
    }

    pub fn add_triplet(&mut self, row: usize, col: usize, val: N) {
        self.row_inds.push(row);
        self.col_inds.push(col);
        self.data.push(val);
    }

    pub fn reserve(&mut self, cap: usize) {
        self.row_inds.reserve(cap);
        self.col_inds.reserve(cap);
        self.data.reserve(cap);
    }

    pub fn reserve_exact(&mut self, cap: usize) {
        self.row_inds.reserve_exact(cap);
        self.col_inds.reserve_exact(cap);
        self.data.reserve_exact(cap);
    }
}

/// Triplet matrix view
pub struct TripletView<'a, N: 'a> {
    rows: usize,
    cols: usize,
    row_inds: &'a [usize],
    col_inds: &'a [usize],
    data: &'a [N],
}

impl<'a, N> TripletView<'a, N> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn nnz(&self) -> usize {
        self.data.len()
    }

    pub fn row_inds(& self) -> &'a [usize] {
        self.row_inds
    }

    pub fn col_inds(& self) -> &'a [usize] {
        self.col_inds
    }

    pub fn data(&self) -> &'a [N] {
        self.data
    }
}