import { cn } from '@/utils/format';

interface ProgressBarProps {
  progress: number;
  className?: string;
  showPercentage?: boolean;
  variant?: 'default' | 'success' | 'danger' | 'warning';
}

export function ProgressBar({ progress, className, showPercentage = true, variant = 'default' }: ProgressBarProps) {
  const clampedProgress = Math.min(100, Math.max(0, progress));

  const variants = {
    default: 'bg-primary-600',
    success: 'bg-green-600',
    danger: 'bg-red-600',
    warning: 'bg-yellow-600',
  };

  return (
    <div className={cn('w-full', className)}>
      <div className="relative h-2 bg-dark-700 rounded-full overflow-hidden">
        <div
          className={cn(
            'h-full rounded-full transition-all duration-300 ease-out',
            variants[variant]
          )}
          style={{ width: `${clampedProgress}%` }}
        />
      </div>
      {showPercentage && (
        <div className="mt-1 text-xs text-gray-400 text-right">
          {clampedProgress.toFixed(1)}%
        </div>
      )}
    </div>
  );
}