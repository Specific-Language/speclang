
// macro_rules! point {
//     (x = $x:expr, y = $y:expr) => {
//         {
//             struct Point {
//                 x: f64,
//                 y: f64
//             }

//             let point = Point {
//                 x: $x,
//                 y: $y
//             };

//             point
//         }
//     };
// }


// #[cfg(test)]
// mod tests {
//     #[test]
//     pub fn test_widget_plugin() {
//         let point = point! {
//             x = 10.0,
//             y = 20.0
//         };

//         assert_eq!(point.x, 10.0);
//         assert_eq!(point.y, 20.0);
//     }
// }
