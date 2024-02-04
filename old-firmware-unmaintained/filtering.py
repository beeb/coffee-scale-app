"""Simple Kalman filter implementation for single-channel feed"""
from math import fabs


class KalmanFilter:
    """Simple Kalman filter for single values."""

    def __init__(self, measurement_uncertainty, q=0.01, estimation_uncertainty=None) -> None:
        """Initialize the filter.

        The initial estimation uncertainty will be equal to the measurement uncertainty if not provided.

        Args:
            measurement_uncertainty (float): how much do we expect our measurement to vary
            q (float, optional): covariance of the process noise, usually between 0.001 and 1. Defaults to 0.01.
            estimation_uncertainty (Optional[float], optional): will be overwritten when we apply the filter. Defaults
                to None.
        """
        self.err_meas = measurement_uncertainty
        self.err_est = estimation_uncertainty or measurement_uncertainty
        self.q = q
        self.last_estimate = 0.0

    def update_estimate(self, measurement) -> float:
        """Perform filtering on the current measurement.

        Args:
            measurement (float): latest measurement

        Returns:
            float: filtered measurement taking into account previous values and trend
        """
        kalman_gain = self.err_est / (self.err_est + self.err_meas)
        current_estimate = self.last_estimate + kalman_gain * (measurement - self.last_estimate)
        self.err_est = (1.0 - kalman_gain) * self.err_est + fabs(self.last_estimate - current_estimate) * self.q
        self.last_estimate = current_estimate

        return current_estimate
