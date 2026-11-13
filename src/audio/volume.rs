// src/audio/volume.rs

/// Керування нелінійною гучністю:
/// До порогу `threshold` (наприклад, 0.7) крок стандартний (0.05).
/// Після досягнення порогу крок зменшується вдвічі (0.025).
#[derive(Debug, Clone, Copy)]
pub struct VolumeControl {
    current: f32,
    threshold: f32,
}

impl VolumeControl {
    pub fn new(initial: f32, threshold: f32) -> Self {
        Self {
            current: initial.clamp(0.0, 1.0),
            threshold: threshold.clamp(0.0, 1.0),
        }
    }

    pub fn value(&self) -> f32 {
        self.current
    }

    pub fn increase(&mut self) -> f32 {
        let step = if self.current >= self.threshold { 0.025 } else { 0.05 };
        self.current = (self.current + step).min(1.0);
        self.current
    }

    pub fn decrease(&mut self) -> f32 {
        let step = if self.current > self.threshold { 0.025 } else { 0.05 };
        self.current = (self.current - step).max(0.0);
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // f32 не можна порівнювати точно: 0.65 + 0.05 != 0.70 у двійковому вигляді
    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 1e-6, "{actual} != {expected}");
    }

    #[test]
    fn test_non_linear_volume_increase() {
        let mut vol = VolumeControl::new(0.65, 0.7);

        // Крок 0.05 до порогу 0.7
        assert_close(vol.increase(), 0.70);

        // Крок 0.025 після перевищення порогу
        assert_close(vol.increase(), 0.725);
    }
}


