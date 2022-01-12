pub fn create_cube(p: f32, l: f32, r: f32,g: f32, b: f32,a: f32) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<u16>){
    let mut pos = Vec::new();
    let mut cor = Vec::new();
    let mut nor = Vec::new();
    let mut idx = Vec::new();

    for i in 0..6 {
        match i {
            //0:前面 1:背面 2:上面 3:底面 4:左側面 5:右側面
            0 => {
                for j in 0..4{
                    match j {
                        //0:原点(左下) 1:左上 2: 右上 3: 右下 
                        0 => {
                            pos.push(p);
                            pos.push(p);
                            pos.push(p + l);
                        }
                        1 => {
                            pos.push(p + l);
                            pos.push(p);
                            pos.push(p + l);
                        }
                        2 => {
                            pos.push(p + l);
                            pos.push(p + l);
                            pos.push(p + l);
                        }
                        3 => {
                            pos.push(p);
                            pos.push(p + l);
                            pos.push(p + l);
                        }
                        _ => {break;}
                    }
                }
            }
            1 => {
                for j in 0..4 {
                    match j {
                        0 => {
                            pos.push(p);
                            pos.push(p);
                            pos.push(p);
                        }
                        1 => {
                            pos.push(p);
                            pos.push(p + l);
                            pos.push(p);
                        }
                        2 => {
                            pos.push(p + l);
                            pos.push(p + l);
                            pos.push(p);
                        }
                        3 => {
                            pos.push(p + l);
                            pos.push(p);
                            pos.push(p);
                        }
                        _ => {break;}
                    }
                }
            }
            2 => {
                for j in 0..4{
                    match j { 
                        0 => {
                            pos.push(p);
                            pos.push(p + l);
                            pos.push(p);
                        }
                        1 => {
                            pos.push(p);
                            pos.push(p + l);
                            pos.push(p + l);
                        }
                        2 => {
                            pos.push(p + l);
                            pos.push(p + l);
                            pos.push(p + l);
                        }
                        3 => {
                            pos.push(p + l);
                            pos.push(p + l);
                            pos.push(p);
                        }
                        _ => {break;}
                    }
                }
            }
            3 => {
                for j in 0..4 {
                    match j {
                        0 => {
                            pos.push(p);
                            pos.push(p);
                            pos.push(p);
                        }
                        1 => {
                            pos.push(p + l);
                            pos.push(p);
                            pos.push(p);
                        }
                        2 => {
                            pos.push(p + l);
                            pos.push(p);
                            pos.push(p + l);
                        }
                        3 => {
                            pos.push(p);
                            pos.push(p);
                            pos.push(p + l);
                        }
                        _ => {break;}
                    }
                }
            }
            4 => {
                for j in 0..4{
                    match j {
                        0 => {
                            pos.push(p);
                            pos.push(p);
                            pos.push(p);
                        }
                        1 => {
                            pos.push(p);
                            pos.push(p);
                            pos.push(p + l);
                        }
                        2 => {
                            pos.push(p);
                            pos.push(p + l);
                            pos.push(p + l);
                        }
                        3 => {
                            pos.push(p);
                            pos.push(p + l);
                            pos.push(p);
                        }
                        _ => {break;}
                    }
                }
            }
            5 => {
                for j in 0..4 {
                    match j {
                        0 => {
                            pos.push(p + l);
                            pos.push(p);
                            pos.push(p);
                        }
                        1 => {
                            pos.push(p + l);
                            pos.push(p + l);
                            pos.push(p);
                        }
                        2 => {
                            pos.push(p + l);
                            pos.push(p + l);
                            pos.push(p + l);
                        }
                        3 => {
                            pos.push(p + l);
                            pos.push(p);
                            pos.push(p + l);
                        }
                        _ => {break;}
                    }
                }
            }
            _ => {break;}
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
            //背面
            else if i == 1 {
                nor.push(0.);
                nor.push(0.);
                nor.push(-1.);

            }
            //上面
            else if i == 2 {
                nor.push(0.);
                nor.push(1.);
                nor.push(0.);
            }
            //底面 
            else if i == 3 {
                nor.push(0.);
                nor.push(-1.);
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
        idx.push(ii + 1);
        idx.push(ii + 2);

        idx.push(ii);
        idx.push(ii + 2);
        idx.push(ii + 3);
        ii += 4;
    }

    (pos, cor, nor, idx)
}