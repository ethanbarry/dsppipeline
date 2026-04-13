%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%------------% SIGNAL PROCESSING PIPELINE MOCKUP %------------%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%%% Written by Ethan Barry <ethanbarry@howdytx.net>
%%% for both MATH 3380 & COSC 4395.
%%%
%%% This file demonstrates the pipeline for processing
%%% noisy signals sampled at a particular bearing into
%%% a correlation score and RSSI value.

fs = 204800;           % Sample rate of the RTL-SDR in Hz.

raw_signal = table2timetable(readtable("cmplxOutputPost.csv"), "SampleRate", fs);
cor_scores = readmatrix("outputCorscores.txt");

% t = (0:length(raw_signal)-1) / fs;
% 
% size(raw_signal)
% size(t)
% 
% %%% Estimate the frequency offset of the fox.
% bins = 2^16;
% X = fftshift(fft(raw_signal, bins));
% freq_axis = linspace(-fs/2, fs/2, bins);
% 
% [~, max_idx] = max(abs(X));
% est_offset = freq_axis(max_idx);
% 
% fprintf("+-- Frequency Report\n");
% %fprintf("|   True Offset:      %.2f Hz\n", offset);
% fprintf("|   Estimated Offset: %.2f Hz\n", est_offset);
% %fprintf("|   Error:            %.2f Hz\n\n", abs(offset - est_offset));
% 
% clean_signal = raw_signal .* exp(-1j * 2 * pi * est_offset * t);
% 
% h = conj(fliplr(burst));
% 
% mf_output = filter(h, 1, clean_signal);
% mf_magnitude = abs(mf_output);
% 
% %%% Making a waterfall plot.
% window_size = 512;
% overlap = 128;
% smallbins = 2^12;
% 
% slices = floor((length(raw_signal) - window_size) / (window_size - overlap));
% 
% waterfall_data = zeros(smallbins, slices);
% time_axis = zeros(1, slices);
% 
% win = hann(window_size);
% win = win(:);
% 
% for idx = 1:slices
%   start_idx = (idx - 1) * (window_size - overlap) + 1;
%   end_idx = start_idx + window_size - 1;
%   chunk = raw_signal(start_idx:end_idx);
%   chunk = chunk(:);
% 
%   W = fftshift(fft(chunk .* win, smallbins));
%   size(waterfall_data(:, idx))
%   size(20 * log10(abs(W) + eps))
%   waterfall_data(:, idx) = 20 * log10(abs(W) + eps);
% 
%   time_axis(idx) = (start_idx + window_size / 2) / fs;
% end
% 
% warning("Plotting")
% 
% 
% %%% Plotting the data.
% figure(1);
% subplot(3, 1, 1);
% plot(freq_axis / 1000, 20 * log10(abs(X) / max(abs(X))));
% title("Frequency Spectrum");
% xlabel("Frequency (kHz)"); ylabel("dB"); grid on;
% 
% subplot(3, 1, 2);
% plot(t * 1e3, imag(clean_signal), 'b');
% title("De-rotated Time Signal");
% xlabel("Time (ms)"); ylabel("Q"); grid on;
% 
% %%% Plotting the waterfall.
% subplot(1, 1, 3);
% freq_axis = linspace(-fs / 2, fs / 2, smallbins) / 1000;
% imagesc(time_axis * 1000, freq_axis, waterfall_data);
% axis yx;
% colorbar;
% title("2D Spectral Waterfall Plot");
% xlabel("Time (ms)");
% ylabel("Frequency Offset (kHz)");
% colormap("jet");
% clim([max(waterfall_data(:)) - 40, max(waterfall_data(:))]);

figure(2)
plot(cor_scores, 1)