(async function renderSuite() {
  const status = document.getElementById("status");
  const summaryDiv = document.getElementById("summary");
  const coverageDiv = document.getElementById("coverage");
  const corpusDiv = document.getElementById("corpus");

  try {
    const res = await fetch("suite_report_data.json");
    if (!res.ok) throw new Error("suite_report_data.json not found");
    const data = await res.json();
    const series = data.suite_series || {};
    const summary = data.suite_summary || {};

    if (!series.elapsed?.length) {
      status.textContent = "No suite samples found.";
      return;
    }

    status.textContent = `${summary.runs ?? 0} run(s) aggregated`;

    // Render summary
    summaryDiv.innerHTML = `<dl>
      <dt>Runs</dt><dd>${summary.runs ?? "N/A"}</dd>
      <dt>Mean coverage</dt><dd>${summary.coverage_mean?.toFixed(2) ?? "N/A"}%</dd>
      <dt>Mean corpus</dt><dd>${summary.corpus_mean?.toFixed(0) ?? "N/A"}</dd>
    </dl>`;

    // Coverage mean curve
    Plotly.newPlot(coverageDiv, [{
      x: series.elapsed,
      y: series.coverage_mean,
      mode: "lines",
      name: "mean",
      line: { width: 2 },
    }], {
      title: "Coverage (%) over time (mean across runs)",
      xaxis: { title: "Elapsed (s)" },
      yaxis: { title: "Coverage (%)" },
      margin: { t: 40, b: 40 },
    }, { responsive: true });

    // Corpus mean curve
    Plotly.newPlot(corpusDiv, [{
      x: series.elapsed,
      y: series.corpus_mean,
      mode: "lines",
      name: "mean",
      line: { width: 2 },
    }], {
      title: "Corpus size over time (mean across runs)",
      xaxis: { title: "Elapsed (s)" },
      yaxis: { title: "Corpus size" },
      margin: { t: 40, b: 40 },
    }, { responsive: true });

  } catch (err) {
    status.textContent = `Error: ${err.message}`;
    console.error(err);
  }
})();
