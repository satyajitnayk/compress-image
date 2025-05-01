document.addEventListener('DOMContentLoaded', () => {
  const fileInput = document.getElementById('fileInput');
  const compressBtn = document.getElementById('compressBtn');
  const resultsDiv = document.getElementById('results');

  compressBtn.addEventListener('click', async () => {
    const file = fileInput.files[0];
    if (!file) {
      alert('Please select an image first');
      return;
    }
    const formData = new FormData();
    formData.append('image', file);

    try {
      const res = await fetch('/compress', {
        method: 'POST',
        body: formData,
      });

      if (!res.ok) throw new Error('compression failed');

      const blob = await res.blob();
      const url = URL.createObjectURL(blob);

      resultsDiv.innerHTML = `
      <h3>Compresses Image </h3>
      <img src="${url}" />
      <a href="${url}" download="compressed.jpg">Download</a>
      `;
    } catch (err) {
      console.log(err);
      alert('compression failed');
    }
  });
});
