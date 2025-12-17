(async function renderRun() {
  const status = document.getElementById("status");
  const summaryDiv = document.getElementById("summary");
  const coverageDiv = document.getElementById("coverage");
  const corpusDiv = document.getElementById("corpus");

  try {
    const res = await fetch("report_data.json");
    if (!res.ok) throw new Error("report_data.json not found");
    const data = await res.json();
    const series = data.series || [];
    const summary = data.summary || {};

    if (!series.length) {
      status.textContent = "No samples found.";
      return;
    }

    status.textContent = `${series.length} CPU(s) tracked`;

    // Render summary
    summaryDiv.innerHTML = `<dl>
      <dt>Total executions</dt><dd>${summary.total_execs?.toLocaleString() ?? "N/A"}</dd>
      <dt>Mean exec/sec</dt><dd>${summary.mean_execs_per_sec?.toFixed(1) ?? "N/A"}</dd>
      <dt>Max coverage</dt><dd>${summary.max_coverage_pct?.toFixed(2) ?? "N/A"}%</dd>
      <dt>Final corpus</dt><dd>${summary.final_corpus_size ?? "N/A"}</dd>
    </dl>`;

    // Coverage chart (per-CPU lines)
    const coverageTraces = series.map((s) => ({
      x: s.elapsed,
      y: s.coverage,
      mode: "lines",
      name: s.cpu,
    }));
    Plotly.newPlot(coverageDiv, coverageTraces, {
      title: "Coverage (%) over time",
      xaxis: { title: "Elapsed (s)" },
      yaxis: { title: "Coverage (%)" },
      legend: { orientation: "h", y: -0.2 },
      margin: { t: 40, b: 60 },
    }, { responsive: true });

    // Corpus chart (per-CPU lines)
    const corpusTraces = series.map((s) => ({
      x: s.elapsed,
      y: s.corpus,
      mode: "lines",
      name: s.cpu,
    }));
    Plotly.newPlot(corpusDiv, corpusTraces, {
      title: "Corpus size over time",
      xaxis: { title: "Elapsed (s)" },
      yaxis: { title: "Corpus size" },
      legend: { orientation: "h", y: -0.2 },
      margin: { t: 40, b: 60 },
    }, { responsive: true });

  } catch (err) {
    status.textContent = `Error: ${err.message}`;
    console.error(err);
  }
})();
