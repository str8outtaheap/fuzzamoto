(async function renderCompare() {
  const status = document.getElementById("status");
  const coverageDiv = document.getElementById("coverage");
  const corpusDiv = document.getElementById("corpus");

  try {
    const res = await fetch("compare_data.json");
    if (!res.ok) throw new Error("compare_data.json not found");
    const data = await res.json();
    const base = data.baseline || {};
    const cand = data.candidate || {};

    if (!base.elapsed?.length || !cand.elapsed?.length) {
      status.textContent = "Insufficient data for comparison.";
      return;
    }

    const mode = data.mode === "suite" ? "Suite" : "Run";
    status.textContent = `${mode} comparison: ${data.baseline_label} vs ${data.candidate_label}`;

    const baseLabel = data.baseline_label || "baseline";
    const candLabel = data.candidate_label || "candidate";

    // Coverage comparison
    Plotly.newPlot(coverageDiv, [
      { x: base.elapsed, y: base.coverage_mean, mode: "lines", name: baseLabel, line: { dash: "dash" } },
      { x: cand.elapsed, y: cand.coverage_mean, mode: "lines", name: candLabel },
    ], {
      title: "Coverage (%) comparison",
      xaxis: { title: "Elapsed (s)" },
      yaxis: { title: "Coverage (%)" },
      legend: { orientation: "h", y: -0.2 },
      margin: { t: 40, b: 60 },
    }, { responsive: true });

    // Corpus comparison
    Plotly.newPlot(corpusDiv, [
      { x: base.elapsed, y: base.corpus_mean, mode: "lines", name: baseLabel, line: { dash: "dash" } },
      { x: cand.elapsed, y: cand.corpus_mean, mode: "lines", name: candLabel },
    ], {
      title: "Corpus size comparison",
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
