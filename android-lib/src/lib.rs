mod client;

use client::{auth, insight, statement, transaction, workspace};
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

fn jstr<'a>(env: &mut JNIEnv<'a>, s: JString<'a>) -> Result<String, String> {
    env.get_string(&s).map(|v| v.into()).map_err(|e| e.to_string())
}

fn opt_jstr<'a>(env: &mut JNIEnv<'a>, s: JString<'a>) -> Option<String> {
    let v: String = env.get_string(&s).ok()?.into();
    if v.is_empty() { None } else { Some(v) }
}

fn out(env: &mut JNIEnv, result: Result<String, String>) -> jstring {
    let s = result.unwrap_or_else(|e| e);
    env.new_string(s).unwrap().into_raw()
}

// ── Auth ─────────────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_register<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, email: JString<'a>, password: JString<'a>, full_name: JString<'a>,
) -> jstring {
    let r = (|| {
        auth::register(&jstr(&mut env, base_url)?, &jstr(&mut env, email)?, &jstr(&mut env, password)?, &jstr(&mut env, full_name)?)
    })();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_login<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, email: JString<'a>, password: JString<'a>,
) -> jstring {
    let r = (|| {
        auth::login(&jstr(&mut env, base_url)?, &jstr(&mut env, email)?, &jstr(&mut env, password)?)
    })();
    out(&mut env, r)
}

// ── Workspaces ────────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_listWorkspaces<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, base_url: JString<'a>, token: JString<'a>,
) -> jstring {
    let r = (|| workspace::list(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?))();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_createWorkspace<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, name: JString<'a>, description: JString<'a>,
) -> jstring {
    let r = (|| {
        let desc = opt_jstr(&mut env, description);
        workspace::create(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, name)?, desc.as_deref())
    })();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_getWorkspace<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
) -> jstring {
    let r = (|| workspace::get(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?))();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_updateWorkspace<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>, name: JString<'a>, description: JString<'a>,
) -> jstring {
    let r = (|| {
        let n = opt_jstr(&mut env, name);
        let d = opt_jstr(&mut env, description);
        workspace::update(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?, n.as_deref(), d.as_deref())
    })();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_deleteWorkspace<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
) -> jstring {
    let r = (|| workspace::delete(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?))();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_listMembers<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
) -> jstring {
    let r = (|| workspace::list_members(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?))();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_addMember<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>, email: JString<'a>,
) -> jstring {
    let r = (|| workspace::add_member(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?, &jstr(&mut env, email)?))();
    out(&mut env, r)
}

// ── Transactions ──────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_listTransactions<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
    category: JString<'a>, tx_type: JString<'a>, from: JString<'a>, to: JString<'a>,
    limit: JString<'a>, offset: JString<'a>,
) -> jstring {
    let r = (|| {
        let f = transaction::TransactionFilter {
            category: None,
            transaction_type: None,
            from: None,
            to: None,
            limit: None,
            offset: None,
        };
        let base = jstr(&mut env, base_url)?;
        let tok = jstr(&mut env, token)?;
        let wid = jstr(&mut env, workspace_id)?;
        let cat = opt_jstr(&mut env, category);
        let ttype = opt_jstr(&mut env, tx_type);
        let fr = opt_jstr(&mut env, from);
        let t = opt_jstr(&mut env, to);
        let lim = opt_jstr(&mut env, limit).and_then(|v| v.parse().ok());
        let off = opt_jstr(&mut env, offset).and_then(|v| v.parse().ok());
        let _ = f;
        transaction::list(&base, &tok, &wid, transaction::TransactionFilter {
            category: cat.as_deref(),
            transaction_type: ttype.as_deref(),
            from: fr.as_deref(),
            to: t.as_deref(),
            limit: lim,
            offset: off,
        })
    })();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_createTransaction<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
    amount: JString<'a>, currency: JString<'a>, category: JString<'a>,
    description: JString<'a>, date: JString<'a>, tx_type: JString<'a>,
) -> jstring {
    let r = (|| {
        let amt: f64 = jstr(&mut env, amount)?.parse().map_err(|_| "invalid amount".to_string())?;
        let desc = opt_jstr(&mut env, description);
        transaction::create(
            &jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?,
            amt, &jstr(&mut env, currency)?, &jstr(&mut env, category)?,
            desc.as_deref(), &jstr(&mut env, date)?, &jstr(&mut env, tx_type)?,
        )
    })();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_deleteTransaction<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>, id: JString<'a>,
) -> jstring {
    let r = (|| transaction::delete(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?, &jstr(&mut env, id)?))();
    out(&mut env, r)
}

// ── Bank Statements ───────────────────────────────────────────────────────────

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_listStatements<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
) -> jstring {
    let r = (|| statement::list(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?))();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_getStatement<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>, id: JString<'a>,
) -> jstring {
    let r = (|| statement::get(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?, &jstr(&mut env, id)?))();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_uploadStatement<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
    file_name: JString<'a>, content_type: JString<'a>, data: JByteArray<'a>,
) -> jstring {
    let r = (|| {
        let bytes = env.convert_byte_array(&data).map_err(|e| e.to_string())?;
        statement::upload(
            &jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?,
            &jstr(&mut env, file_name)?, &jstr(&mut env, content_type)?, &bytes,
        )
    })();
    out(&mut env, r)
}

// ── AI Insights ───────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_listInsights<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>, insight_type: JString<'a>,
) -> jstring {
    let r = (|| {
        let t = opt_jstr(&mut env, insight_type);
        insight::list(&jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?, t.as_deref())
    })();
    out(&mut env, r)
}

#[no_mangle]
pub extern "system" fn Java_dev_msdqn_finance_FinanceSdk_generateInsights<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>,
    base_url: JString<'a>, token: JString<'a>, workspace_id: JString<'a>,
    date_from: JString<'a>, date_to: JString<'a>,
) -> jstring {
    let r = (|| insight::generate(
        &jstr(&mut env, base_url)?, &jstr(&mut env, token)?, &jstr(&mut env, workspace_id)?,
        &jstr(&mut env, date_from)?, &jstr(&mut env, date_to)?,
    ))();
    out(&mut env, r)
}
