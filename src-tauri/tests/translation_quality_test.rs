//! 翻訳品質評価統合テスト
//!
//! 30サンプルのテストケースを使用して翻訳品質を測定し、
//! 目標スコア80/100以上を達成していることを確認します。

use honnyaku_lib::llm::claude_cli::translate_with_claude_cli;
use honnyaku_lib::services::translation::Language;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio::process::Command;

/// テストケース構造
#[derive(Debug, Deserialize)]
struct TestCase {
    id: u32,
    category: String,
    source_text: String,
    source_lang: String,
    target_lang: String,
    expected_translation: String,
}

/// テストケース集合
#[derive(Debug, Deserialize)]
struct TestCases {
    test_cases: Vec<TestCase>,
}

/// 評価結果
#[derive(Debug, Serialize)]
struct EvaluationResult {
    id: u32,
    category: String,
    source_text: String,
    actual_translation: String,
    expected_translation: String,
    terminology_score: f32,    // 専門用語の正確性 (30点)
    context_score: f32,         // 文脈理解と一貫性 (25点)
    expression_score: f32,      // 自然な表現 (25点)
    structure_score: f32,       // 構造とフォーマットの保持 (10点)
    accuracy_score: f32,        // 技術的正確性 (10点)
    total_score: f32,           // 合計 (100点)
}

/// 評価レポート
#[derive(Debug, Serialize)]
struct EvaluationReport {
    total_samples: usize,
    average_score: f32,
    category_scores: Vec<CategoryScore>,
    individual_results: Vec<EvaluationResult>,
}

/// カテゴリ別スコア
#[derive(Debug, Serialize)]
struct CategoryScore {
    category: String,
    count: usize,
    average: f32,
    min: f32,
    max: f32,
}

/// 言語文字列をLanguage enumに変換
fn parse_language(lang: &str) -> Language {
    match lang.to_lowercase().as_str() {
        "en" | "english" => Language::English,
        "ja" | "japanese" => Language::Japanese,
        _ => Language::English,
    }
}

/// Claude CLIを使用して翻訳結果を評価
async fn evaluate_translation(
    source_text: &str,
    expected: &str,
    actual: &str,
    source_lang: &str,
    _target_lang: &str,
) -> Result<(f32, f32, f32, f32, f32), String> {
    let evaluation_prompt = format!(
        r#"以下の翻訳を評価してください。

原文 ({}): {}

期待される翻訳: {}

実際の翻訳: {}

以下の5項目を評価し、JSON形式で点数を返してください：

1. terminology_score (0-30点): 専門用語の正確性
   - 技術用語の正しい翻訳
   - 専門用語の一貫性

2. context_score (0-25点): 文脈理解と一貫性
   - 代名詞・接続詞の適切な翻訳
   - 文間の論理的なつながり

3. expression_score (0-25点): 自然な表現
   - ネイティブスピーカーにとっての自然さ
   - 文体の統一

4. structure_score (0-10点): 構造とフォーマットの保持
   - コードブロック、APIエンドポイントの保持
   - リスト構造の維持

5. accuracy_score (0-10点): 技術的正確性
   - 技術的な意味の正確さ
   - 誤訳の有無

JSON形式で返してください：
{{
  "terminology_score": <0-30の数値>,
  "context_score": <0-25の数値>,
  "expression_score": <0-25の数値>,
  "structure_score": <0-10の数値>,
  "accuracy_score": <0-10の数値>
}}

数値のみを返し、説明は不要です。"#,
        source_lang, source_text, expected, actual
    );

    // Claude CLIで評価を実行
    let output = Command::new("claude")
        .arg("-p")
        .arg("--output-format")
        .arg("json")
        .arg(&evaluation_prompt)
        .output()
        .await
        .map_err(|e| format!("評価実行エラー: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "評価コマンド失敗: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let response = String::from_utf8_lossy(&output.stdout);

    // Claude CLIのJSON出力をパース
    #[derive(Deserialize)]
    struct ClaudeResponse {
        result: String,
    }

    let claude_response: ClaudeResponse = serde_json::from_str(&response)
        .map_err(|e| format!("Claude CLIレスポンス解析エラー: {} - 応答: {}", e, response))?;

    // resultフィールドからJSONを抽出（マークダウンコードブロックの中にある可能性がある）
    let result_text = &claude_response.result;
    let json_str = if let Some(start) = result_text.find('{') {
        if let Some(end) = result_text.rfind('}') {
            &result_text[start..=end]
        } else {
            result_text
        }
    } else {
        result_text
    };

    // JSONをパース
    #[derive(Deserialize)]
    struct Scores {
        terminology_score: f32,
        context_score: f32,
        expression_score: f32,
        structure_score: f32,
        accuracy_score: f32,
    }

    let scores: Scores = serde_json::from_str(json_str)
        .map_err(|e| format!("スコアJSON解析エラー: {} - 応答: {}", e, json_str))?;

    Ok((
        scores.terminology_score,
        scores.context_score,
        scores.expression_score,
        scores.structure_score,
        scores.accuracy_score,
    ))
}

#[tokio::test]
#[ignore] // 実際のClaude CLIを使用するため、手動実行が必要
async fn test_translation_quality_evaluation() {
    // テストケースJSONを読み込む
    let test_cases_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("translation_quality_test_cases.json");

    let content = fs::read_to_string(&test_cases_path)
        .expect("テストケースファイルが読み込めません");

    let test_cases: TestCases =
        serde_json::from_str(&content).expect("JSON解析エラー");

    println!("=== 翻訳品質評価テスト開始 ===");
    println!("テストケース数: {}", test_cases.test_cases.len());

    let mut results = Vec::new();

    // 各テストケースを処理
    for (index, test_case) in test_cases.test_cases.iter().enumerate() {
        println!(
            "\n[{}/{}] ID: {} - カテゴリ: {}",
            index + 1,
            test_cases.test_cases.len(),
            test_case.id,
            test_case.category
        );

        let source_lang = parse_language(&test_case.source_lang);
        let target_lang = parse_language(&test_case.target_lang);

        // 翻訳を実行
        print!("  翻訳実行中...");
        let translation_result =
            translate_with_claude_cli(&test_case.source_text, source_lang, target_lang, None)
                .await;

        let actual_translation = match translation_result {
            Ok(result) => {
                println!(" 完了 ({}ms)", result.duration_ms);
                result.translated_text
            }
            Err(e) => {
                println!(" エラー: {}", e);
                format!("翻訳エラー: {}", e)
            }
        };

        // 評価を実行
        print!("  評価実行中...");
        let (terminology, context, expression, structure, accuracy) =
            match evaluate_translation(
                &test_case.source_text,
                &test_case.expected_translation,
                &actual_translation,
                &test_case.source_lang,
                &test_case.target_lang,
            )
            .await
            {
                Ok(scores) => {
                    println!(" 完了");
                    scores
                }
                Err(e) => {
                    println!(" エラー: {}", e);
                    // エラー時はデフォルトスコア（50点相当）
                    (15.0, 12.5, 12.5, 5.0, 5.0)
                }
            };

        let total = terminology + context + expression + structure + accuracy;

        println!(
            "  スコア: {:.1}/100 (専門用語:{:.1}, 文脈:{:.1}, 表現:{:.1}, 構造:{:.1}, 正確性:{:.1})",
            total, terminology, context, expression, structure, accuracy
        );

        results.push(EvaluationResult {
            id: test_case.id,
            category: test_case.category.clone(),
            source_text: test_case.source_text.clone(),
            actual_translation,
            expected_translation: test_case.expected_translation.clone(),
            terminology_score: terminology,
            context_score: context,
            expression_score: expression,
            structure_score: structure,
            accuracy_score: accuracy,
            total_score: total,
        });
    }

    // 統計を計算
    let average_score: f32 = results.iter().map(|r| r.total_score).sum::<f32>()
        / results.len() as f32;

    // カテゴリ別統計
    let categories = vec![
        "api_documentation",
        "error_message",
        "technical_blog",
        "code_comment",
    ];
    let mut category_scores = Vec::new();

    for category in categories {
        let category_results: Vec<_> = results
            .iter()
            .filter(|r| r.category == category)
            .collect();

        if !category_results.is_empty() {
            let avg = category_results
                .iter()
                .map(|r| r.total_score)
                .sum::<f32>()
                / category_results.len() as f32;
            let min = category_results
                .iter()
                .map(|r| r.total_score)
                .fold(f32::INFINITY, f32::min);
            let max = category_results
                .iter()
                .map(|r| r.total_score)
                .fold(f32::NEG_INFINITY, f32::max);

            category_scores.push(CategoryScore {
                category: category.to_string(),
                count: category_results.len(),
                average: avg,
                min,
                max,
            });
        }
    }

    // レポートを生成
    let report = EvaluationReport {
        total_samples: results.len(),
        average_score,
        category_scores,
        individual_results: results,
    };

    // レポートを保存
    let report_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docs")
        .join("quality-test-results");

    fs::create_dir_all(&report_path).expect("レポートディレクトリ作成エラー");

    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let report_file = report_path.join(format!("{}-evaluation.json", timestamp));

    fs::write(
        &report_file,
        serde_json::to_string_pretty(&report).expect("JSON変換エラー"),
    )
    .expect("レポート保存エラー");

    // 結果サマリーを出力
    println!("\n=== 評価結果サマリー ===");
    println!("総サンプル数: {}", report.total_samples);
    println!("平均スコア: {:.1}/100", average_score);
    println!("\nカテゴリ別スコア:");
    for cat in &report.category_scores {
        println!(
            "  {}: {:.1}/100 (最低:{:.1}, 最高:{:.1}, サンプル数:{})",
            cat.category, cat.average, cat.min, cat.max, cat.count
        );
    }
    println!("\nレポート保存先: {}", report_file.display());

    // アサーション: 平均スコアが80以上であることを確認
    assert!(
        average_score >= 80.0,
        "平均スコアが目標の80/100未満です: {:.1}/100",
        average_score
    );

    println!("\n✅ 品質目標達成: {:.1}/100 (目標: 80/100以上)", average_score);
}
