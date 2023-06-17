extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use config::Config;
use std::collections::HashMap;
use ansi_term::enable_ansi_support;


//項目が含まれているかの評価関数
fn ins_mch(tgt: &str,val: &str) -> bool{
    if tgt.matches(val).collect::<Vec<&str>>().len() > 0{
        return true;
    }else{
        return false;
    }
}

//項目が含まれていないかの評価関数
fn ins_not_mch(tgt: &str,val: &str) -> bool{
    if tgt.matches(val).collect::<Vec<&str>>().len() > 0{
        return false;
    }else{
        return true;
    }
}

//すべての項目が存在するかの評価関数
fn ins_existall(tgt: &str, val: &str) -> bool{
    dbg!(tgt,val);
    let mut result: bool = true; //AND式なのでtrueで初期化
    for node in val.split("/").collect::<Vec<&str>>() {
        dbg!(node);
        let mut is_node_ok = false; //ここはOR条件
        for tgt_split in tgt.split(" ").collect::<Vec<&str>>(){
            dbg!(tgt_split);
            if tgt_split == node{
                is_node_ok = true;
            }
        }
        result = result && is_node_ok;
        dbg!(is_node_ok);
    }
    dbg!(result);
    return result;
}

//すべての項目が存在しないかの評価関数
fn ins_not_existall(tgt: &str, val: &str) -> bool{
    dbg!(tgt,val);
    let mut result: bool = true; //AND式なのでtrueで初期化
    for node in val.split("/").collect::<Vec<&str>>() {
        dbg!(node);
        let mut is_node_ok = false; //ここはOR条件
        for tgt_split in tgt.split(" ").collect::<Vec<&str>>(){
            dbg!(tgt_split);
            if tgt_split == node{
                is_node_ok = true;
            }
        }
        result = result && is_node_ok;
        dbg!(is_node_ok);
    }
    dbg!(result);
    return !result;
}

//少なくとも一つの項目が存在するかの評価関数
fn ins_atleastone(tgt: &str, val: &str) -> bool{
    dbg!(tgt,val);
    let mut result: bool = false; //OR式なのでfalseで初期化
    for node in val.split("/").collect::<Vec<&str>>() {
        dbg!(node);
        let mut is_node_ok = false; //ここはOR条件
        for tgt_split in tgt.split(" ").collect::<Vec<&str>>(){
            dbg!(tgt_split);
            if tgt_split == node{
                is_node_ok = true;
            }
        }
        result = result || is_node_ok;
        dbg!(is_node_ok);
    }
    dbg!(result);
    return result;
}

//少なくとも一つの項目が存在しないかの評価関数
fn ins_notatleastone(tgt: &str, val: &str) -> bool{
    dbg!(tgt,val);
    let mut result: bool = false; //OR式なのでfalseで初期化
    for node in val.split("/").collect::<Vec<&str>>() {
        dbg!(node);
        let mut is_node_ok = false; //ここはOR条件
        for tgt_split in tgt.split(" ").collect::<Vec<&str>>(){
            dbg!(tgt_split);
            if tgt_split == node{
                is_node_ok = true;
            }
        }
        result = result || is_node_ok;
        dbg!(is_node_ok);
    }
    dbg!(result);
    return !result;
}

fn ins_exist(tgt: &str, val: &str) -> bool{
    return true;
}

fn cfgchk<F: Fn(&str,&str) -> bool>(func: F, lines: &str, dir: &str, val: &str, logic: &str) -> bool {
    let mut ctr: u16 = 0; //スキャンカウント
    let mut line_no: u16 = 0; //行番号
    let mut result: bool;
    let re = Regex::new(r"#.*$").unwrap(); //コメント消す用の正規表現
    
    println!("{:?}ディレクティブをスキャンします",dir);
    
    if logic == "OR" || logic == "EXIST"{
        result = false; //ORの時はfalseにすれば影響を与えない
    }else{ //想定外の値の場合、より厳しい判定のANDになる(安全策)
        result = true; //ANDの時はtrueにすれば影響を与えない
    }


    //1行づつ処理を行う
    for line in lines.replace("\r","").split("\n"){
        line_no += 1;
        //空白文字で分割
        let line_split: Vec<&str> = line.split(" ").collect();
        
        //有効なノードが今まであったかのフラグ(フラグ自体なくてもいい気がするけど安全策)
        let mut ever_seen_flag: bool = false;
        
        //空白で分割された要素をnodeとして処理
        for node in line_split {
          if node != "" && ever_seen_flag == false {
              //初めて見つかった有効なノード = ディレクティブと扱う
              dbg!("有効なノードが初めて見つかりました!");
              ever_seen_flag = true;
              
              //調査対象ディレクティブと一致する場合はターゲットとする
              if node == dir {
                  dbg!("ターゲットのディレクティブでした。評価関数に渡します。");
                  
                  ctr += 1;
                  let tgt = re.replace(line,"").replace("\"","").to_uppercase();

                  let func_res: bool = func(&tgt,val);
                  if func_res {
                      println!("{}行:\x1b[92m{}\x1b[0m",line_no,line);
                  }else{
                      println!("{}行:\x1b[91m{}\x1b[0m",line_no,line);
                  }

                  if logic == "OR" || logic == "EXIST"{
                      result = func_res || result;
                  }else{ //想定外の値の場合、より厳しい判定のANDになる(安全策)
                      result = func_res && result;
                  }
                  
                  break;
              }else if node.replace("#","") == dir{
                  dbg!("ターゲットのディレクティブですが全体がコメントアウトされています");
                  println!("{}行:\x1b[90m{}\x1b[0m",line_no,line);
                  break;
              }else{
                  dbg!("関係ないディレクティブなのでこの行はスキップします");
                  break;
              }
          }
        }
    }

    if ctr == 0 {
        println!("指定されたディレクティブは存在しませんでした");
        return false;
    }else{
        return result;
    }
}


fn scan(descr: &str, dir: &str, val: &str, logic: &str, ins: &str) {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("引数の数が正しくありません");
        return;
    }

    let mut f = File::open(&args[1]).expect("file not found");
    
    let mut lines = String::new();
    f.read_to_string(&mut lines)
        .expect("something went wrong reading the file");
    
    let mut _res: bool;
    
    println!("{}",descr);
    match ins {
        "match" => _res = cfgchk(ins_mch,&lines,dir,val,logic),
        "not_match" => _res = cfgchk(ins_not_mch,&lines,dir,val,logic),
        "exist_all" => _res = cfgchk(ins_existall,&lines,dir,val,logic),
        "not_exist_all" => _res = cfgchk(ins_not_existall,&lines,dir,val,logic),
        "at_least_one" => _res = cfgchk(ins_atleastone,&lines,dir,val,logic),
        "not_at_least_one" => _res = cfgchk(ins_notatleastone,&lines,dir,val,logic),
        "exist" => _res = cfgchk(ins_exist,&lines,dir,val,logic),
        "not_exist" => _res = !cfgchk(ins_exist,&lines,dir,val,logic),
        &_ => {_res = false; println!("命令が間違っています")}
    }
    
    match _res {
        true => println!("\x1b[92m-> pass!\x1b[0m\n"),
        false => println!("\x1b[91m-> fail!\x1b[0m\n")
    }
    dbg!(_res);

}

fn main() {
    //WindowsでANSIエスケープシーケンスを利用できるようにする
    let enabled = ansi_term::enable_ansi_support();

    //ポリシーの読み込み
    let fp = Config::builder()
        .add_source(config::File::with_name("./policy.toml"))
        .build()
        .unwrap();
    let policies = fp.try_deserialize::<HashMap<String,HashMap<String,String>>>().unwrap();
    let mut policies_s: Vec<_> = policies.iter().collect::<Vec<_>>();
    policies_s.sort_by(|a, b| a.0.cmp(&b.0));

    for policy in policies_s{
        scan(&policy.1["description"], &policy.1["directive"], &policy.1["value"].to_uppercase(), &policy.1["logic"], &policy.1["instruction"])
    }

}