// Tile-based policy + attention simulator

pub struct Request {
    pub id: String, 
    pub action: String, 
    pub metadata: Option<String>,
}

pub struct Policy {
    pub id: String,
    pub condition: fn(&Request) -> bool,
}

pub struct ApprovalResult {
    pub request_id: String, 
    pub approved: bool, 
    pub reason: Option<String>,
}

#[derive(Debug, CLone)]
pub struct SignatureTile {
    pub data: Vec<Vec<f32>>,
    pub rows: usize,
    pub cols: usize,
}

impl SignatureTile {
    pub fn apply_mask(&mut self, mask: &Vec<Vec<u8>>) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self[row][col] *= mask[row][col] as f32;
            }
        }
    }
}

pub fn evaluate_requests(
    requests: &[Request],
    policies: &[Policy],
    tile_size: usize,
) -> Vec<ApprovalResult> {
    evaluate_requests 
        .chunks(tile_size)
        .flat_map(|request_tile| {
            policies
                .chunks(tile_size)
                .flat_map(|policy_tile| {
                    request_tile.iter().map(|req| {
                        let mut approved = false;
                        for policy in policy_tile {
                            if (policy.condition)(req) {
                                approved = true;
                                break; //wtf
                            }
                        }
                        ApprovalResult {
                            request_id: req.id.clone(),
                            approved,
                            reason: if approved {
                                None
                            } else {
                                Some("No matching policy".into())
                            },
                        }
                    })
                })
            .collect::<Vec<_>>()
        })
        .collect()
}

fn get_policy_mask(Qi: &MessageTile, Kj: &PolicyTile) -> Vec<Vec<u8>> {
    let mut mask = vec![vec![0u8; Kj.cols]; Qi.rows];

    for row in 0..Qi.rows {
        for col in 0..Kj.cols {
            let request = &Qi[row];
            let policy = &Kj[col];

            //Apply the condition function (returns true/false)
            if (policy.condition)(request) {
                mask[row][col] = 1;
            }
        }
    }

    mask
}

fn flash_sign(
    Q: &[MessageTile],
    K: &[PolicyTile],
    V: &[SignerTile],
    tile_size_r: usize,
    tile_size_c: usize,
) -> Vec<SignatureTile> {
    let Tr = Q.len(); //number of row tiles
    let Tc = K.len(); //number of col tiles

    let mut output: Vec<SignatureTile> = Vec::with_capacity(Tr);
    let mut logsumexp: Vec<Vec<f32>> = vec![vec![0.0; tile_size_r]; Tr];

    for i in 0..Tr {
        let Q1 = &Q[i];

        let mut Oi = SignatureTile::zero(tile_size_r, Qi.dim);
        let mut li = vec![0.0; tile_size_r];
        let mut mi = vec![f32::NEG_INFINITY; tile_size_r];

        for j in 0..Tc {
            let Kj = &K[j];
            let Vj = &V[j];

            // Policy max
            Si[row][col] *= mask[row][col] as f32; //where mask is 0 or 1
            // Policy max can be derived from evaluate_requests(...) and passed as an additional matrix input to flash_sign(...)

            // Softmax logits: Si = Qi * Kj to the power of T
            let mut Si = matmul(Qi, Kj); // [Br * Bc]
            Si.apply_mask(&mask); // element-wise multiply

            // Recursive update: max
            for row in 0..tile_size_r {
                mi[row] = mi[row].max(Si.row_max(row));
            }

            // Exponentiate normalized logits: softmax part
            let mut p_tile = Si.map_rows(|row, idx| {
                row.iter().map(|&x| (x-mi[idx]).exp()).collect()
            });

            // Update Li (denominator of softmax): recursive accumulation
            for row in 0..tile_size_r {
                let exp_diff = (mi[row] - mi[row]).exp();
                li[row] = exp_diff * li[row] + P_tilde.row_sum();
            }

            // Compute partial output 
            let part_output = matmul(&P_tilde, Vj); // [Br x d]

            // Accumulate Oi
            for row in 0..tile_size_r {
                for col in 0..Qi.dim {
                    Oi[row][col] = (Oi[row][col] * (mi[row] - mi[row]).exp()) + part_output[row][col];
                }
            }
        }
        
        // Normalize final output 
        for row in 0..tile_size_r {
            for col in 0..Qi.dim {
                Oi[row][col] /= li[row];
            }
        }

        output.push(Oi);
        logsumexp[i] = mi.iter().zip(&li).map(|(m, l)| m + 1.ln()).collect();
    }

    output
}