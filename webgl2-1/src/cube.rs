pub fn create_cube(p: f32, r: f32,g: f32, b: f32,a: f32) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<u16>){
    let mut pos = Vec::new();
    let mut cor = Vec::new();
    let mut nor = Vec::new();
    let mut idx = Vec::new();
    let mut cx: f32;
    let mut cy: f32;
    let mut cz: f32;

    for i in 0..6 {
        //前面&底面
        if i == 0 || i == 1 {
            for j in 0..4{
                if j == 0 || j == 3{
                    cx = -p;
                } else {
                    cx = p;
                }

                if j < 2 || i == 1{
                    cy = -p;
                } else {
                    cy = p;
                }

                if i == 0 || j > 1{
                    cz = p;
                } else {
                    cz = -p;
                }

                pos.push(cx);
                pos.push(cy);
                pos.push(cz);
            }
        //背面&上面
        }else if i == 2 || i == 3{
            for j in 0..4{
                if j < 2 {
                    cx = -p;
                } else {
                    cx = p;
                }

                if (i == 2 && j == 1) || (i == 2 && j == 2) {
                    cy = p;
                } else {
                    cy = -p;
                }

                if (i == 3 && j == 1) || (i == 3 && j == 2) {
                    cz = p;
                } else {
                    cz = -p;
                }

                pos.push(cx);
                pos.push(cy);
                pos.push(cz);
            }
        //右側面&左側面
        }else if i == 4 || i == 5 {
            for j in 0..4 {
                if i == 4 {
                    cx = p;
                } else {
                    cx = -p;
                }

                if j == 0 || i == 5 && j == 1 || i == 4 && j == 3 {
                    cy = -p;
                } else {
                    cy = p;
                }

                if j == 0 || i == 4 && j == 1 || i == 5 && j == 3 {
                    cz = -p;
                } else {
                    cz = p;
                }

                pos.push(cx);
                pos.push(cy);
                pos.push(cz);
            }
        }
    }

    //カラーバッファ
    for i in 0..6 {
        for j in 0..4{  
            cor.push(r);
            cor.push(g);
            cor.push(b);
            cor.push(a);
        }
    }

    //法線座標
    for i in 0..6 {
        for j in 0..4 {
            //前面
            if i == 0 {
                nor.push(0.);
                nor.push(0.);
                nor.push(1.);
            }
            //底面
            else if i == 1 {
                nor.push(0.);
                nor.push(-1.);
                nor.push(0.);

            }
            //背面 
            else if i == 2 {
                nor.push(0.);
                nor.push(0.);
                nor.push(-1.);
            }
            //上面 
            else if i == 3 {
                nor.push(0.);
                nor.push(1.);
                nor.push(0.);
            }
            //右側面 
            else if i == 4 {
                nor.push(1.);
                nor.push(0.);
                nor.push(0.);
            } 
            //左側面
            else if i == 5 {
                nor.push(-1.);
                nor.push(0.);
                nor.push(0.);
            }
        }
    }

    let mut ii = 0;
    //index
    for i in 0..6 {
        idx.push(ii);
        idx.push(ii+1);
        idx.push(ii+2);

        idx.push(ii);
        idx.push(ii+2);
        idx.push(ii+3);
        ii += 4;
    }

    (pos, cor, nor, idx)
}