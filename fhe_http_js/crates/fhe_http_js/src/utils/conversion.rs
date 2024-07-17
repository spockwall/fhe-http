use neon::prelude::*;

#[allow(unused)]
pub(crate) fn vec_u8_to_array<'a, C: Context<'a>>(
    vec: &Vec<u8>,
    cx: &mut C,
) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as usize);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.number(*s);
        a.set(cx, i as u32, v)?;
    }
    Ok(a)
}

#[allow(unused)]
pub(crate) fn array_to_vec_u8<'a, C: Context<'a>>(arr: Handle<JsArray>, cx: &mut C) -> Vec<u8> {
    let vec = arr.to_vec(cx).unwrap();

    vec.iter()
        .map(|v| {
            let num: Handle<'a, JsNumber> = v.downcast(cx).unwrap();
            num.value(cx) as u8
        })
        .collect()
}
