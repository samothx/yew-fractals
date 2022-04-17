Has anyone tried working with the clipboard api  in web-sys ? I am trying to copy the contents of a canvas to the clipboard.
What I am doing is thew following: 
- get pixel data from canvasses 2D context
- convert to png
- create a web-sys::Blob with type image/png from png data
-  create a ClipboardItem from the blob (this is where things seem to go sideways)
-  create an array of ClipboardItems containg only the one item 
- check if I have permission to access clipboard
- call clipboard::write with the array created above. 

Code looks like this:
```
    pub fn copy_to_clipboard(&self) {
        // TODO: understand Promises & Closures in web-sys
        info!("copy_to_clipboard: entered, preparing imagedata");
        let performance = window()
            .expect("Window not found")
            .performance()
            .expect("performance should be available");

        let start = performance.now();

        let image_data = self.get_2d_context()
            .get_image_data(
                0.0,
                0.0,
                self.canvas.width().into(),
                self.canvas.height().into(),
            )
            .expect("failed to retrieve image data")
            .dyn_into::<ImageData>()
            .expect("Failed to cast to ImageData").data().clone();

        info!("copy_to_clipboard: converting to png at offset: {:.3} secs", (performance.now() - start) / 1000.0);
        let mut img_u8: Vec<u8> = Vec::new();
        let items = match write_rgba_from_u8(&mut img_u8,
                                             &(*image_data)[..],
                                             self.canvas.width(),
                                             self.canvas.height()) {
            Ok(()) => {
                // Create an U8Array
                info!("copy_to_clipboard: creating JsArray from png at offset: {:.3} secs", (performance.now() - start) / 1000.0);
                let u8_array = Array::new();
                img_u8.iter().for_each(|val| {
                    u8_array.push(&JsValue::from(*val));
                });
                // Create Blob from type & U8Array
                let mut options = BlobPropertyBag::new();
                options.type_("image/png");
                info!("copy_to_clipboard: creating Blob from JsArray at offset: {:.3} secs", (performance.now() - start) / 1000.0);
                let blob = Blob::new_with_u8_array_sequence_and_options(&JsValue::from(u8_array), &options)
                    .expect("Failed to create blob");
                // Create ClipboardItem from Blob
                info!("copy_to_clipboard: creating ClipboardItem from Blob  at offset: {:.3} secs", (performance.now() - start) / 1000.0);
                let items_obj = Object::new();
                Reflect::set(&items_obj, &blob.type_().into(), &blob)
                    .expect("Failed to write blob to object ");
                let item = ClipboardItem::from(JsValue::from(items_obj));
                info!("copy_to_clipboard: got ClipboardItem: {:?}", item);
                // Create array of ClipboardItems
                info!("copy_to_clipboard: creating Array of ClipboardItems at offset: {:.3} secs", (performance.now() - start) / 1000.0);
                let items = js_sys::Array::new();
                items.push(&item);
                items
            },
            Err(err) => {
                error!("Failed to create png from imagedata, error: {:?}", err);
                return;
            }
        };

        info!("copy_to_clipboard: creating query for clipboard permissions at offset: {:.3} secs", (performance.now() - start) / 1000.0);
        let query_obj = Object::new();
        Reflect::set(&query_obj, &JsValue::from("name"), &&JsValue::from("clipboard-write"))
            .expect("Failed to write blob to object ");

        // info!("copy_to_clipboard: got query_object: {:?}", query_obj);
        match window().expect("Window not found")
            .navigator()
            .permissions()
            .expect("no permissions found in navigator")
            .query(&query_obj)
        {
            Ok(result) => {
                info!("copy_to_clipboard: query permission returned ok");
                spawn_local(async move {
                    let query_res = JsFuture::from(result)
                        .await
                        .expect("Query promise was rejected");
                    let status = PermissionStatus::from(query_res);
                    // info!("copy_to_clipboard: got PermissionStatus {:?}", status.state());
                    if status.state() == PermissionState::Granted {
                        info!("copy_to_clipboard: got permission to copy to clipboard");

                        let clipboard = window().expect("Window not found")
                            .navigator()
                            .clipboard().expect("Clipboard not found");
                        match JsFuture::from(clipboard.write(&items)).await {
                            Ok(_res) => info!("copy_to_clipboard: png image copied to clipboard"),
                            Err(err) => error!("copy_to_clipboard: failed to copy png image to clipboard, error: {:?}", err)
                        }
                    } else {
                        warn!("copy_to_clipboard: not permitted to copy to clipboard: {:?}", status.state());
                    }
                });
            }
            Err(err) => {
                warn!(
                    "copy_to_clipboard: error from query permissions, msg: {}",
                    err.as_string().unwrap_or("None".to_string())
                );
            }
        }
    }

```
