use std::fs;
use std::path::Path;

use rlua::{
    Context as LuaContext, Error as LuaError, Function as LuaFunction, Result as LuaResult,
    String as LuaString, Table as LuaTable, Value as LuaValue,
};

use crate::content::manifest::NeocivManifest;

pub fn load_content_file<'lua>(ctx: LuaContext<'lua>) -> LuaResult<LuaFunction<'lua>> {
    ctx.create_function(|fn_ctx, filename: String| {
        // Confirm the file exists
        let cf_path = Path::new(filename.as_str());

        if cf_path.exists() && !cf_path.is_dir() {
            // Get path to parent directory of content file and then ensure it is absolute
            let base_path = cf_path
                .parent()
                .expect("Unable to get parent path to content file")
                .canonicalize()
                .expect("Unable to get absolute path to content file directory")
                .as_path()
                .to_owned();

            // Get the path to the manifest
            let manifest = match NeocivManifest::from_child_path_str(base_path.to_str().unwrap()) {
                Some(m) => m,
                None => panic!("Unable find manifest"),
            };

            // Read the file into JSON
            let json_src = fs::read_to_string(cf_path).unwrap();

            // Get reference to lunajson
            let fn_require: LuaFunction = fn_ctx.globals().get("require")?;
            let lunajson_decode: LuaFunction =
                fn_require.call::<_, LuaTable>("lunajson")?.get("decode")?;

            // Turn into a lua table
            let mut content_tbl: LuaTable = lunajson_decode.call::<_, LuaTable>(json_src)?;

            // Turn into an array if not already one
            if content_tbl.len()? == 0 {
                let arr_table = fn_ctx.create_table()?;
                arr_table.set(0, content_tbl)?;
                content_tbl = arr_table;
            }

            // Iterate over each entry in the table by index - going by "pairs" doesn't work
            // so well as it moves the table making it cumbersome to return
            for i in 1..content_tbl.len()? + 1 {
                let content: LuaTable = content_tbl.get(i)?;

                // Replace "@" in the ID with the manfiest ID
                let content_id = content.get::<_, LuaString>("id")?;
                if content_id.to_str().unwrap().starts_with("@") {
                    let new_id = content_id
                        .to_str()
                        .unwrap()
                        .replace("@", manifest.id.as_str());
                    content.set("id", new_id)?;
                }

                let resources: LuaTable = content.get("resources")?;
                let new_resources = fn_ctx.create_table()?;

                // Iterate over all the entries in the resource table and create absolute paths
                // to each one, except for the materials which are organised by namespace
                for rpair in resources.pairs::<LuaString, LuaValue>() {
                    let pair = rpair?;
                    let key = pair.0;
                    if key.eq(&String::from("materials")) {
                        let materials: LuaTable = fn_ctx.unpack::<LuaTable>(pair.1)?;
                        for n in 1..materials.len()? + 1 {
                            let material_str = materials.get::<_, LuaString>(n)?;
                            let m_id = material_str
                                .to_str()
                                .unwrap()
                                .replace("@", manifest.id.as_str());
                            materials.set(n, m_id)?;
                        }
                        new_resources.set::<_, LuaTable>(key, materials)?;
                    } else {
                        // Coerce the Lua String containing the resource path into a Rust String
                        let resource_str = fn_ctx
                            .coerce_string(pair.1)
                            .unwrap()
                            .expect("Unable to coerce resource value");

                        // Create a new Path buffer from the provided resource String
                        let resource_path = Path::new(resource_str.to_str().unwrap());

                        // Join that to the absolute path of the directory
                        // containing the content file
                        let new_path = fn_ctx
                            .create_string(base_path.join(resource_path).to_str().unwrap())?;
                        // Set this value in the new resources table
                        new_resources.set::<_, LuaString>(key, new_path)?;
                    }
                }

                content.set::<_, LuaTable>("resources", new_resources)?;
            }

            return LuaResult::Ok(content_tbl);
        } else {
            panic!("Failed to find content file '{}'", filename);
        }
    })
}
