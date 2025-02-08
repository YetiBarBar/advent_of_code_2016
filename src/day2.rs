fn main() {
    let data = r"URULLLLLRLDDUURRRULLLDURRDRDRDLURURURLDLLLLRUDDRRLUDDDDDDLRLRDDDUUDUDLDULUDLDURDULLRDDURLLLRRRLLRURLLUDRDLLRRLDDRUDULRRDDLUUUDRLDLURRRULURRDLLLDDDLUDURDDRLDDDLLRULDRUDDDLUDLURUDLLRURRUURUDLLLUUUUDDURDRDDDLDRRUDURDLLLULUDURURDUUULRULUDRUUUUDLRLUUUUUDDRRDDDURULLLRRLDURLDLDRDLLLUULLRRLLLLDRLRDRRDRRUDDLULUUDDDDRRUUDDLURLRDUUDRRLDUDLRRRLRRUUDURDRULULRDURDRRRDLDUUULRDDLRLRDLUUDDUDDRLRRULLLULULLDDDRRDUUUDDRURDDURDRLRDLDRDRULRLUURUDRLULRLURLRRULDRLRDUDLDURLLRLUDLUDDURDUURLUDRLUL
LLLUUURUULDDDULRRDLRLLLLLLLLRURRDLURLUDRRDDULDRRRRRRLDURRULDDULLDDDRUUDLUDULLDLRRLUULULRULURDURLLDULURDUDLRRLRLLDULLRLDURRUULDLDULLRDULULLLULDRLDLDLDLDDLULRLDUDRULUDDRDDRLRLURURRDULLUULLDRRDRRDLDLLRDLDDUUURLUULDDRRRUULDULDDRDDLULUDRURUULLUDRURDRULDRUULLRRDURUDDLDUULLDDRLRRDUDRLRRRLDRLRULDRDRRUDRLLLDDUDLULLURRURRLUURDRLLDLLDUDLUUURRLRDDUDRLUDLLRULLDUUURDLUUUDUDULRLDLDRUUDULRDRRUDLULRLRDLDRRDDDUDLDLDLRUURLDLLUURDLDLRDLDRUDDUURLLLRDRDRRULLRLRDULUDDDLUDURLDUDLLRULRDURDRDLLULRRDLLLDUURRDUDDLDDRULRRRRLRDDRURLLRRLLL
DRURLDDDDRLUDRDURUDDULLRRLLRLDDRLULURLDURRLDRRLRLUURDDRRDLRDLDLULDURUDRLRUDULRURURLRUDRLLDDUDDRDLDRLLDDLRRDRUUULDUUDRUULRLLDLLULLLRRDRURDLDDRRDDUDDULLDUUULDRUDLDLURLDRURUDLRDDDURRLRDDUDLLLRRUDRULRULRRLLUUULDRLRRRLLLDLLDUDDUUDRURLDLRRUUURLUDDDRRDDLDDDDLUURDDULDRLRURLULLURRDRLLURLLLURDURLDLUDUUDUULLRLDLLLLULRDDLDUDUDDDUULURRLULDLDRLRDRLULLUDDUUUUURDRURLDUULDRRDULUDUDLDDRDLUDDURUDURLDULRUDRRDLRLRDRRURLDLURLULULDDUUDLRLLLLURRURULDDRUUULLDULDRDULDDDLLLRLULDDUDLRUDUDUDURLURLDDLRULDLURD
DRUDRDURUURDLRLUUUUURUDLRDUURLLDUULDUULDLURDDUULDRDDRDULUDDDRRRRLDDUURLRDLLRLRURDRRRDURDULRLDRDURUDLLDDULRDUDULRRLLUDLLUUURDULRDDLURULRURDDLRLLULUDURDRRUDLULLRLDUDLURUDRUULDUDLRDUDRRDULDDLDRLRRULURULUURDULRRLDLDULULRUUUUULUURLURLRDLLRRRRLURRUDLRLDDDLDRDRURLULRDUDLRLURRDRRLRLLDLDDLLRRULRLRLRUDRUUULLDUULLDDRLUDDRURLRLDLULDURLLRRLDLLRDDDUDDUULLUDRUDURLLRDRUDLUDLLUDRUUDLRUURRRLLUULLUUURLLLRURUULLDLLDURUUUULDDDLRLURDRLRRRRRRUDLLLRUUULDRRDLRDLLDRDLDDLDLRDUDLDDRDDDDRULRRLRDULLDULULULRULLRRLLUURUUUDLDLUDUDDDLUUDDDDUDDDUURUUDRDURRLUULRRDUUDDUDRRRDLRDRLDLRRURUUDRRRUUDLDRLRDURD
DDDLRURUDRRRURUUDLRLRDULDRDUULRURRRUULUDULDDLRRLLRLDDLURLRUDRLRRLRDLRLLDDLULDLRRURDDRDLLDDRUDRRRURRDUDULUDDULRRDRLDUULDLLLDRLUDRDURDRRDLLLLRRLRLLULRURUUDDRULDLLRULDRDLUDLULDDDLLUULRRLDDUURDLULUULULRDDDLDUDDLLLRRLLLDULRDDLRRUDDRDDLLLLDLDLULRRRDUDURRLUUDLLLLDUUULDULRDRULLRDRUDULRUUDULULDRDLDUDRRLRRDRLDUDLULLUDDLURLUUUDRDUDRULULDRDLRDRRLDDRRLUURDRULDLRRLLRRLDLRRLDLDRULDDRLURDULRRUDURRUURDUUURULUUUDLRRLDRDLULDURUDUDLUDDDULULRULDRRRLRURLRLRLUDDLUUDRRRLUUUDURLDRLRRDRRDURLLL";

    let mut last_touch2 = PadTouch2::Touch5;
    let mut last_touch1 = PadTouch1::Touch5;
    for line in data.lines() {
        last_touch1 = PadTouch::process_line(last_touch1, line);
        last_touch2 = PadTouch::process_line(last_touch2, line);
        println!("{last_touch1:?} \t\t {last_touch2:?}");
    }
}

pub trait PadTouch
where
    Self: Sized,
{
    fn process_line(init_value: Self, input: &str) -> Self {
        input.chars().fold(init_value, |touch, chr| match chr {
            'U' => touch.move_up(),
            'D' => touch.move_down(),
            'L' => touch.move_left(),
            'R' => touch.move_right(),
            _ => touch,
        })
    }
    #[must_use]
    fn move_down(&self) -> Self;
    #[must_use]
    fn move_up(&self) -> Self;
    #[must_use]
    fn move_right(&self) -> Self;
    #[must_use]
    fn move_left(&self) -> Self;
}

#[derive(Debug)]
enum PadTouch1 {
    Touch1 = 1,
    Touch2,
    Touch3,
    Touch4,
    Touch5,
    Touch6,
    Touch7,
    Touch8,
    Touch9,
}

#[derive(Debug)]
pub enum PadTouch2 {
    Touch1,
    Touch2,
    Touch3,
    Touch4,
    Touch5,
    Touch6,
    Touch7,
    Touch8,
    Touch9,
    TouchA,
    TouchB,
    TouchC,
    TouchD,
}

impl PadTouch for PadTouch2 {
    /*
          1
        2 3 4
      5 6 7 8 9
        A B C
          D
    */
    fn move_down(&self) -> Self {
        match self {
            PadTouch2::Touch1 => PadTouch2::Touch3,
            PadTouch2::Touch2 => PadTouch2::Touch6,
            PadTouch2::Touch3 => PadTouch2::Touch7,
            PadTouch2::Touch4 => PadTouch2::Touch8,
            PadTouch2::Touch5 => PadTouch2::Touch5,
            PadTouch2::Touch6 | PadTouch2::TouchA => PadTouch2::TouchA,
            PadTouch2::Touch7 => PadTouch2::TouchB,
            PadTouch2::Touch8 | PadTouch2::TouchC => PadTouch2::TouchC,
            PadTouch2::Touch9 => PadTouch2::Touch9,
            PadTouch2::TouchB | PadTouch2::TouchD => PadTouch2::TouchD,
        }
    }

    fn move_up(&self) -> Self {
        match self {
            PadTouch2::Touch1 | PadTouch2::Touch3 => PadTouch2::Touch1,
            PadTouch2::Touch2 | PadTouch2::Touch6 => PadTouch2::Touch2,
            PadTouch2::Touch8 | PadTouch2::Touch4 => PadTouch2::Touch4,
            PadTouch2::Touch5 => PadTouch2::Touch5,
            PadTouch2::Touch7 => PadTouch2::Touch3,
            PadTouch2::Touch9 => PadTouch2::Touch9,
            PadTouch2::TouchA => PadTouch2::Touch6,
            PadTouch2::TouchB => PadTouch2::Touch7,
            PadTouch2::TouchC => PadTouch2::Touch8,
            PadTouch2::TouchD => PadTouch2::TouchB,
        }
    }
    /*
          1
        2 3 4
      5 6 7 8 9
        A B C
          D
    */
    fn move_left(&self) -> Self {
        match self {
            PadTouch2::Touch1 => PadTouch2::Touch1,
            PadTouch2::Touch3 | PadTouch2::Touch2 => PadTouch2::Touch2,
            PadTouch2::Touch4 => PadTouch2::Touch3,
            PadTouch2::Touch6 | PadTouch2::Touch5 => PadTouch2::Touch5,
            PadTouch2::Touch7 => PadTouch2::Touch6,
            PadTouch2::Touch8 => PadTouch2::Touch7,
            PadTouch2::Touch9 => PadTouch2::Touch8,
            PadTouch2::TouchB | PadTouch2::TouchA => PadTouch2::TouchA,
            PadTouch2::TouchC => PadTouch2::TouchB,
            PadTouch2::TouchD => PadTouch2::TouchD,
        }
    }
    /*
          1
        2 3 4
      5 6 7 8 9
        A B C
          D
    */
    fn move_right(&self) -> Self {
        match self {
            PadTouch2::Touch1 => PadTouch2::Touch1,
            PadTouch2::Touch2 => PadTouch2::Touch3,
            PadTouch2::Touch4 | PadTouch2::Touch3 => PadTouch2::Touch4,
            PadTouch2::Touch5 => PadTouch2::Touch6,
            PadTouch2::Touch6 => PadTouch2::Touch7,
            PadTouch2::Touch7 => PadTouch2::Touch8,
            PadTouch2::Touch9 | PadTouch2::Touch8 => PadTouch2::Touch9,
            PadTouch2::TouchA => PadTouch2::TouchB,
            PadTouch2::TouchC | PadTouch2::TouchB => PadTouch2::TouchC,
            PadTouch2::TouchD => PadTouch2::TouchD,
        }
    }
}

impl PadTouch for PadTouch1 {
    fn move_up(&self) -> Self {
        match self {
            PadTouch1::Touch4 | PadTouch1::Touch1 => PadTouch1::Touch1,
            PadTouch1::Touch5 | PadTouch1::Touch2 => PadTouch1::Touch2,
            PadTouch1::Touch6 | PadTouch1::Touch3 => PadTouch1::Touch3,
            PadTouch1::Touch7 => PadTouch1::Touch4,
            PadTouch1::Touch8 => PadTouch1::Touch5,
            PadTouch1::Touch9 => PadTouch1::Touch6,
        }
    }

    fn move_down(&self) -> Self {
        match self {
            PadTouch1::Touch1 => PadTouch1::Touch4,
            PadTouch1::Touch2 => PadTouch1::Touch5,
            PadTouch1::Touch3 => PadTouch1::Touch6,
            PadTouch1::Touch7 | PadTouch1::Touch4 => PadTouch1::Touch7,
            PadTouch1::Touch8 | PadTouch1::Touch5 => PadTouch1::Touch8,
            PadTouch1::Touch9 | PadTouch1::Touch6 => PadTouch1::Touch9,
        }
    }

    fn move_left(&self) -> Self {
        match self {
            PadTouch1::Touch2 | PadTouch1::Touch1 => PadTouch1::Touch1,
            PadTouch1::Touch3 => PadTouch1::Touch2,
            PadTouch1::Touch5 | PadTouch1::Touch4 => PadTouch1::Touch4,
            PadTouch1::Touch6 => PadTouch1::Touch5,
            PadTouch1::Touch8 | PadTouch1::Touch7 => PadTouch1::Touch7,
            PadTouch1::Touch9 => PadTouch1::Touch8,
        }
    }
    fn move_right(&self) -> Self {
        match self {
            PadTouch1::Touch1 => PadTouch1::Touch2,
            PadTouch1::Touch3 | PadTouch1::Touch2 => PadTouch1::Touch3,
            PadTouch1::Touch4 => PadTouch1::Touch5,
            PadTouch1::Touch6 | PadTouch1::Touch5 => PadTouch1::Touch6,
            PadTouch1::Touch7 | PadTouch1::Touch8 => PadTouch1::Touch8,
            PadTouch1::Touch9 => PadTouch1::Touch9,
        }
    }
}
